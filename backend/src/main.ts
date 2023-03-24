import {
  Application as OakApplication,
  Router as OakRouter,
  Response as OakResponse,
  version,
} from "../deps.ts";
import con from "./common/console.ts";
import { exitError } from "./common/errors.ts";
import {
  generateProviderTree,
  parseStationIdentifiers,
  ProviderTree,
} from "./common/providers.ts";
import { getJSON as _getJSON } from "./common/fetch.ts";
import { fetchStations, StationList, StationQuery } from "./common/stations.ts";
import {
  CommonDepartureList,
  startDepartureUpdate,
} from "./common/departures.ts";
import { getEnv } from "./common/env.ts";

con.info(`Abfahrten Server - version ${version}\n`);

const env = getEnv();

const rawStationIdentifiers = env.stationIdentifiers.split(";");

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

let stations: StationList;
try {
  stations = await fetchStations(
    stationQueries,
    providerTree,
    env.useFirstResult
  );
} catch (e) {
  exitError(e);
}

const departures: CommonDepartureList = {};
for (const stationId in stations) {
  const station = stations[stationId];
  departures[stationId] = { departures: [], notifications: [] };
  startDepartureUpdate(
    env.fetchInterval,
    departures[stationId],
    station,
    providerTree
  );
}

const useCors = (res: OakResponse) =>
  res.headers.set("Access-Control-Allow-Origin", env.cors);

const useJsonResponse = (res: OakResponse) => {
  useCors(res);
  res.type = "json";
};

const router = new OakRouter();
router.get(`${env.baseUrl}/available`, (ctx) => {
  useJsonResponse(ctx.response);
  // Oak seems to filter all spicy parts of providerTree
  ctx.response.body = { stations: stations, provider: providerTree };
});

router.get(`${env.baseUrl}/departures/:stationId`, (ctx) => {
  useJsonResponse(ctx.response);
  if (ctx.params.stationId in stations) {
    ctx.response.body = departures[ctx.params.stationId];
  } else {
    ctx.response.status = 404;
  }
});

router.get(`${env.baseUrl}/stations/:stationId`, (ctx) => {
  useJsonResponse(ctx.response);
  if (ctx.params.stationId in stations) {
    ctx.response.body = stations[ctx.params.stationId];
  } else {
    ctx.response.status = 404;
  }
});

const app = new OakApplication();
app.use(router.routes());
app.use(router.allowedMethods());
app.listen({ port: env.port });
console.log(""); // TODO: evtl. in con.info optionale linebreaks einführen
con.info(`Server listening on port ${env.port}; base URL: "${env.baseUrl}"`);
con.info(`fetching new data every ${env.fetchInterval}s\n`);
