import con from "./console.ts";
import { Departure, Notification, VehicleType } from "./departures.ts";
import { CompError, ErrorType } from "./errors.ts";
import { LineList } from "./line.ts";
import { Station, StationIdList } from "./stations.ts";
import { StationQuery } from "./stations.ts";

/**
 * Common interface describing all providers
 */
export interface Provider {
  name: string;
  findStationId(searchQuery: string): Promise<StationIdList>;
  getCurrentDepartures(
    station: Station,
    lines: LineList
  ): Promise<{
    departures: Departure[];
    notifications: Notification[];
  }>;
  getAvailableLines(stationId: string): Promise<LineList>;
}

/**
 * Parses a raw station string and extracts the required provider and provider-specific station identifier
 * @param station station identifier: `<Country>:<City>:<Provider>@station` or `<Country>:<City>@station`
 * @returns parsed data
 */
function parseStationIdentifier(station: string): StationQuery {
  const separatorIndex = station.indexOf("@");
  if (separatorIndex === -1) {
    throw new CompError(ErrorType.BAD_REQUEST, "Invalid station identifier");
  }

  const providerMeta = station.substring(0, separatorIndex).split(":");

  if (providerMeta.length < 2 || providerMeta.length > 3) {
    throw new CompError(ErrorType.BAD_REQUEST, "Invalid provider identifier");
  }

  const country = providerMeta[0];
  const city = providerMeta[1];
  const provider = providerMeta.length === 3 ? providerMeta[2] : "default";

  return {
    country: country,
    city: city,
    provider: provider,
    stationQuery: station.substring(separatorIndex + 1, station.length),
  };
}

/**
 * Parses a list of raw station identifiers strings and maps them to a list of Station objects
 * @param stations A list of strings with the raw station identifier strings
 * @returns Parsed list of stations
 */
export function parseStationIdentifiers(stations: string[]) {
  const parsedStations: StationQuery[] = [];
  for (const station of stations) {
    try {
      parsedStations.push(parseStationIdentifier(station));
    } catch (e) {
      throw e;
    }
  }
  return parsedStations;
}

/**
 * Type for all country meta.ts files
 */
export type CountryMeta = {
  name: string;
};

export type LineConfig = {
  [type in VehicleType]: {
    [name: string]: { fontColor: string; background: string };
    default: { fontColor: string; background: string };
  };
};

/**
 * Type for all city meta.ts files
 */
export type CityMeta = {
  name: string;
  defaultProvider: string;
  lineConfig: LineConfig;
};

/**
 * Type of the providerTree describing all used providers
 */
export type ProviderTree = {
  [country: string]: {
    cities: {
      [city: string]: {
        providers: {
          [provider: string]: Provider;
        };
      } & CityMeta;
    };
  } & CountryMeta;
};

/**
 * Generates a tree of all the required providers and loads the corresponding files.
 * It also replaces "default" providers in the station list
 * @param stations A list of all the stations the backend will provide
 * @returns generated ProviderTree
 */
export async function generateProviderTree(
  stations: StationQuery[]
): Promise<ProviderTree> {
  const providers: ProviderTree = {};

  for (const station of stations) {
    // Add country if necessary
    if (!(station.country in providers)) {
      let countryMeta: CountryMeta;
      try {
        countryMeta = await import(`../providers/${station.country}/meta.ts`);
      } catch {
        throw new CompError(
          ErrorType.NOT_FOUND,
          `Country: "${station.country}" not found`
        );
      }
      providers[station.country] = { cities: {}, name: countryMeta.name };
    }

    const providerTreeCurrentCountry = providers[station.country];

    // Add city if necessary
    if (!(station.city in providerTreeCurrentCountry.cities)) {
      let cityMeta: CityMeta;

      try {
        cityMeta = await import(
          `../providers/${station.country}/${station.city}/meta.ts`
        );
      } catch {
        throw new CompError(
          ErrorType.NOT_FOUND,
          `City: "${station.country}:${station.city}" not found`
        );
      }

      providerTreeCurrentCountry.cities[station.city] = {
        providers: {},
        name: cityMeta.name,
        defaultProvider: cityMeta.defaultProvider,
        lineConfig: cityMeta.lineConfig,
      };
    }

    const providerTreeCurrentCity =
      providerTreeCurrentCountry.cities[station.city];

    // Add provider if necessary
    if (station.provider === "default") {
      con.warn(
        `using default provider "${station.country}:${station.city}:${providerTreeCurrentCity.defaultProvider}" for "${station.country}:${station.city}", because no specific provider has been provided`
      );
      station.provider = providerTreeCurrentCity.defaultProvider;
    }
    if (!(station.provider in providerTreeCurrentCity)) {
      let providerModule: { Provider: { new (): Provider } };

      try {
        providerModule = await import(
          `../providers/${station.country}/${station.city}/${station.provider}/provider.ts`
        );
      } catch {
        throw new CompError(
          ErrorType.NOT_FOUND,
          `Provider: "${station.country}:${station.city}:${station.provider}" not found`
        );
      }

      providerTreeCurrentCity.providers[station.provider] =
        new providerModule.Provider();
    }
  }
  return providers;
}
