import useSWR, { SWRResponse } from "swr";
import { DepartureList } from "../types/api";
import { ExtendedSettings } from "../types/settings";

export function useDepartures(settings: ExtendedSettings) {
  let key = "";
  let fetcher: ((args: string) => Promise<DepartureList>) | null = null;
  if (settings) {
    key = `${settings.serverURL}/api/v1/departures/${settings.station}`;
    fetcher = async (args: string): Promise<DepartureList> => {
      try {
        return (await fetch(args)).json();
      } catch (err) {
        throw err;
      }
    };
  }

  return useSWR(key, fetcher);
}
