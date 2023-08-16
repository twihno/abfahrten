import { VehicleType } from "./departures.ts";

export type Line = {
  name: string;
  direction: string;
  id: string;
  type: VehicleType;
};

export type LineList = {
  [id: string]: Line;
};
