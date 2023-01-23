import { version } from "../deps.ts";
import con from "./common/console.ts";
import { ErrorType, exitError } from "./common/errors.ts";
import {
  generateProviderTree,
  parseStationIdentifiers,
  ProviderTree,
} from "./common/providers.ts";
import { getJSON as _getJSON } from "./common/fetch.ts";
import { fetchStations, Station, StationQuery } from "./common/stations.ts";

con.info(`Abfahrten Server - version ${version}\n`);

const envStationIdentifiers = Deno.env.get("STATIONS");

if (envStationIdentifiers === undefined || !envStationIdentifiers.trim()) {
  exitError({ type: ErrorType.BAD_REQUEST, msg: "No stations defined" });
}

const rawStationIdentifiers = envStationIdentifiers.split(";");

let stationQueries: StationQuery[] = [];
try {
  stationQueries = parseStationIdentifiers(rawStationIdentifiers);
} catch (e) {
  exitError(e);
}

let providerTree: ProviderTree | undefined;
try {
  providerTree = await generateProviderTree(stationQueries);
} catch (e) {
  exitError(e);
}

let stations: Station[];
try {
  stations = await fetchStations(stationQueries, providerTree, true);
} catch (e) {
  exitError(e);
}

console.log(stations);
