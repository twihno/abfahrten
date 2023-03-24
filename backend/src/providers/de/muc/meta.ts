import { VehicleType } from "../../../common/departures.ts";
import { LineConfig } from "../../../common/providers.ts";

export const defaultProvider = "mvv";
export const name = "Munich";

export const lineConfig: LineConfig = {
  "0": {
    U1: { background: "red", fontColor: "green" },
    default: { background: "blue", fontColor: "red" },
  },
};
