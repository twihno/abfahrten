import { DateTime } from "luxon";

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

export type DepartureList = {
  departures: Departure[];
  notifications: Notification[];
};
