import { DateTime } from "../../deps.ts";
import con from "./console.ts";
import { printError } from "./errors.ts";
import { Provider, ProviderTree } from "./providers.ts";
import { Station } from "./stations.ts";

export const enum VehicleType {
  U_BAHN,
  S_BAHN,
  TRAM,
  BUS,
  EXPRESS_BUS,
  UNDEFINED,
}

export type Departure = {
  direction: string;
  lineName: string;
  lineId: string;
  trackLabel: string;
  track: string;
  departurePlanned: DateTime;
  departureLive: DateTime;
  onTime: boolean;
  notification: boolean;
};

export type Notification = {
  text: string;
  lines: string[];
};

export type CommonDepartureList = {
  [station: string]: DepartureList;
};

export type DepartureList = {
  departures: Departure[];
  notifications: Notification[];
};

export async function getCurrentDeparture(
  provider: Provider,
  station: Station
): Promise<DepartureList> {
  let currentDepartures: Departure[];
  let currentNotifications: Notification[];

  try {
    ({ departures: currentDepartures, notifications: currentNotifications } =
      await provider.getCurrentDepartures(station, station.availableLines));
  } catch (e) {
    throw e;
  }

  if (currentDepartures.length === 0) {
    return {
      departures: [],
      notifications: [],
    };
  }

  return {
    departures: currentDepartures.sort((a: Departure, b: Departure) => {
      return a.departureLive.diff(b.departureLive).toObject().milliseconds!;
    }),
    notifications: currentNotifications,
  };
}

export async function updateDeparture(
  _departures: DepartureList,
  station: Station,
  providerTree: ProviderTree
) {
  let newDepartureList: DepartureList = { departures: [], notifications: [] };
  try {
    newDepartureList = await getCurrentDeparture(
      providerTree[station.country].cities[station.city].providers[
        station.provider
      ],
      station
    );
    con.info(`updated ${station.internalId} - ${station.name}`);
  } catch (e) {
    printError(e);
    _departures = { departures: [], notifications: [] };
    return;
  }

  _departures.departures = newDepartureList.departures;
  _departures.notifications = newDepartureList.notifications;
}

export async function startDepartureUpdate(
  interval: number,
  departures: DepartureList,
  station: Station,
  providerTree: ProviderTree
) {
  await updateDeparture(departures, station, providerTree);
  setInterval(
    updateDeparture,
    interval * 1000,
    departures,
    station,
    providerTree
  );
}
