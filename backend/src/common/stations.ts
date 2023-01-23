import con from "./console.ts";
import { CompError, ErrorType } from "./errors.ts";
import { ProviderTree } from "./providers.ts";

export type StationQuery = {
  country: string;
  city: string;
  provider: string;
  stationQuery: string;
  stationId: string | undefined;
  stationName: string | undefined;
};

export type Station = {
  country: string;
  city: string;
  provider: string;
  stationId: string;
  stationName: string;
};

export type StationIdList = {
  name: string;
  id: string;
}[];

export async function fetchStations(
  stations: StationQuery[],
  providerTree: ProviderTree,
  useFirstResult: boolean
): Promise<Station[]> {
  const fetchedStations: Station[] = [];

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
        `No matching station for "${station.country}:${station.city}:${station.provider}@${station.stationQuery}"`
      );
    }
    if (potentialStations.length > 1) {
      con.warn(
        `multiple candidates for "${station.country}:${station.city}:${station.provider}@${station.stationQuery}"`
      );

      for (const opt of potentialStations) {
        con.info(` - "${opt.name}" - id: "${opt.id}"`);
      }

      if (!useFirstResult) {
        throw new CompError(
          ErrorType.USER_INPUT_REQUIRED,
          "Can't continue with multiple possible options for this station." +
            'Please change the id in your config to the desired one or set USE_FIRST to "true"'
        );
      }
    }

    fetchedStations.push({
      country: station.country,
      city: station.city,
      provider: station.provider,
      stationId: potentialStations[0].id,
      stationName: potentialStations[0].name,
    });

    con.info(
      `using station "${potentialStations[0].name}" for "${station.country}:${station.city}:${station.provider}@${station.stationQuery}"`
    );
  }

  return fetchedStations;
}
