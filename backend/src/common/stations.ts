import con from "./console.ts";
import { CompError, ErrorType } from "./errors.ts";
import { ProviderTree } from "./providers.ts";
import { LineList } from "./line.ts";

export type StationQuery = {
  country: string;
  city: string;
  provider: string;
  stationQuery: string;
};

export type Station = {
  country: string;
  city: string;
  provider: string;
  id: string;
  internalId: string;
  name: string;
  availableLines: LineList;
};

export type StationList = {
  [id: string]: Station;
};

export type StationIdList = {
  name: string;
  id: string;
}[];

export async function fetchStations(
  stations: StationQuery[],
  providerTree: ProviderTree,
  useFirstResult: boolean
): Promise<StationList> {
  const fetchedStations: StationList = {};

  for (const station of stations) {
    const provider =
      providerTree[station.country].cities[station.city].providers[
        station.provider
      ];
    let potentialStations: StationIdList = [];
    try {
      potentialStations = await provider.findStationId(station.stationQuery);
    } catch (e) {
      throw e;
    }
    if (potentialStations.length === 0) {
      throw new CompError(
        ErrorType.NOT_FOUND,
        `no matching station for "${station.country}:${station.city}:${station.provider}@${station.stationQuery}"`
      );
    }
    if (potentialStations.length > 1) {
      con.warn(
        `multiple candidates for "${station.country}:${station.city}:${station.provider}@${station.stationQuery}"`
      );

      for (const opt of potentialStations) {
        con.warn(` - "${opt.name}" - id: "${opt.id}"`);
      }

      if (!useFirstResult) {
        throw new CompError(
          ErrorType.USER_INPUT_REQUIRED,
          "Can't continue with multiple possible options for this station. " +
            'Please change the id in your config to the desired one or set USE_FIRST_RESULT to "true"'
        );
      }

      con.warn("using first candidate because USE_FIRST_RESULT=true");
    }

    const stationId = potentialStations[0].id;
    const stationName = potentialStations[0].name;

    let availableLines: LineList | undefined;
    try {
      availableLines = await provider.getAvailableLines(stationId);
    } catch (e) {
      throw e;
    }

    const internalId = `${station.country}:${station.city}:${station.provider}@${stationId}`;

    fetchedStations[internalId] = {
      country: station.country,
      city: station.city,
      provider: station.provider,
      id: stationId,
      name: stationName,
      internalId: `${station.country}:${station.city}:${station.provider}@${stationId}`,
      availableLines: availableLines,
    };

    con.info(
      `using station "${potentialStations[0].name}" for "${station.country}:${station.city}:${station.provider}@${station.stationQuery}"`
    );
  }

  return fetchedStations;
}
