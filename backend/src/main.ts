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

// Load settings in environment variables
const env = getEnv();

// Split station identifiers listed in env variable
const rawStationIdentifiers = env.stationIdentifiers.split(";");

// Parse station identifiers
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

const stationsOverview: {
  id: string;
  country: string;
  city: string;
  name: string;
  abfahrtenId: string;
}[] = [];

for (const station in stations) {
  stationsOverview.push({
    id: stations[station].id,
    country: stations[station].country,
    city: stations[station].city,
    name: stations[station].name,
    abfahrtenId: stations[station].internalId,
  });
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

router.get(`${env.baseUrl}/v1/departures/:stationId`, (ctx) => {
  useJsonResponse(ctx.response);
  if (ctx.params.stationId in stations) {
    ctx.response.body = departures[ctx.params.stationId];
  } else {
    ctx.response.status = 404;
  }
});

router.get(`${env.baseUrl}/v1/stations`, (ctx) => {
  useJsonResponse(ctx.response);

  ctx.response.body = stationsOverview;
});

router.get(`${env.baseUrl}/v1/stations/:stationId`, (ctx) => {
  useJsonResponse(ctx.response);
  if (ctx.params.stationId in stations) {
    ctx.response.body = stations[ctx.params.stationId];
  } else {
    ctx.response.status = 404;
  }
});

router.get(`${env.baseUrl}/v1/city/:stationId`, (ctx) => {
  useJsonResponse(ctx.response);
  if (!providerTree) {
    ctx.response.status = 500;
  } else if (ctx.params.stationId in stations) {
    const tmpStation = stations[ctx.params.stationId];
    const city = providerTree[tmpStation.country].cities[tmpStation.city];
    ctx.response.body = {
      name: city.name,
      city: tmpStation.city,
      country: tmpStation.country,
      lineConfigs: city.lineConfig,
    };
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
