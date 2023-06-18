import { VehicleType } from "../../../common/departures.ts";
import { LineConfig } from "../../../common/providers.ts";

export const defaultProvider = "mvv";
export const name = "Munich";

export const lineConfig: LineConfig = {
  [VehicleType.U_BAHN]: {
    U1: { background: "#4d753c", fontColor: "#fff", radius: "m" },
    U2: { background: "#b52036", fontColor: "#fff", radius: "m" },
    U3: { background: "#dd6e2f", fontColor: "#fff", radius: "m" },
    U4: { background: "#4fab88", fontColor: "#fff", radius: "m" },
    U5: { background: "#b57f1b", fontColor: "#fff", radius: "m" },
    U6: { background: "#356ab0", fontColor: "#fff", radius: "m" },
    U7: {
      background:
        "linear-gradient(to bottom right, #5f8536 0 50%, #b52036 50% 100%)",
      fontColor: "#fff",
      radius: "m",
    },
    U8: {
      background:
        "linear-gradient(to bottom right, #b52036 0 50%, #dd6e2f 50% 100%)",
      fontColor: "#fff",
      radius: "m",
    },
    default: { fontColor: "#fff", background: "#0068b0", radius: "m" },
  },
  [VehicleType.S_BAHN]: {
    S1: {
      fontColor: "#fff",
      background: "#1ab3e2",
      radius: "f",
    },
    S2: {
      fontColor: "#fff",
      background: "#71bf44",
      radius: "f",
    },
    S3: {
      fontColor: "#fff",
      background: "#7b107d",
      radius: "f",
    },
    S4: {
      fontColor: "#fff",
      background: "#ee1c25",
      radius: "f",
    },
    S6: {
      fontColor: "#fff",
      background: "#008a51",
      radius: "f",
    },
    S7: {
      fontColor: "#fff",
      background: "#963833",
      radius: "f",
    },
    S8: {
      fontColor: "#ffcb06",
      background: "#000",
      radius: "f",
    },
    S20: {
      fontColor: "#fff",
      background: "#f05a73",
      radius: "f",
    },
    default: {
      fontColor: "#fff",
      background: "#008e4e",
      radius: "f",
    },
  },
  [VehicleType.TRAM]: {
    default: {
      fontColor: "#fff",
      background: "#d91a1a",
      radius: "0",
    },
  },
  [VehicleType.BUS]: {
    default: {
      fontColor: "#fff",
      background: "#0b576a",
      radius: "f",
    },
  },
  [VehicleType.EXPRESS_BUS]: {
    default: {
      fontColor: "#fff",
      background: "#4e7e6c",
      radius: "f",
    },
  },
  [VehicleType.UNDEFINED]: {
    default: {
      fontColor: "#000",
      background: "#fff",
      radius: "m",
    },
  },
};
