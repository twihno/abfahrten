import { BadRequestError } from "../base/errors.ts";

export type StationQuery = {
  country: string;
  city: string;
  provider: string | null;
  stationQuery: string;
};

/**
 * Parses a raw station string and extracts the required provider and provider-specific station identifier
 * @param station station identifier: `<Country>:<City>:<Provider>@station` or `<Country>:<City>@station`
 * @returns parsed data
 */
function parseStationIdentifier(station: string): StationQuery {
  const separatorIndex = station.indexOf("@");
  if (separatorIndex === -1) {
    throw new BadRequestError("Invalid station identifier");
  }

  const providerId = station.substring(0, separatorIndex).split(":");

  // Allowed length: 2 or 3 (2: use default provider for city)
  if (providerId.length < 2 || providerId.length > 3) {
    throw new BadRequestError("Invalid provider identifier");
  }

  const country = providerId[0];
  const city = providerId[1];
  const provider = providerId.length === 3 ? providerId[2] : "default";

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
