import useSWR, { SWRResponse } from "swr";
import { DepartureList } from "../types/api";
import { ExtendedSettings } from "../types/settings";

export function useDepartures(
  settings: ExtendedSettings
): SWRResponse<DepartureList, Error, any> {
  let key = "";
  let fetcher: ((args: string) => Promise<DepartureList>) | null = null;
  if (settings) {
    key = `${settings.serverURL}/api/v1/departures/${settings.station}`;
    fetcher = async (args: string): Promise<DepartureList> => {
      console.debug("fetch");
      let res;
      try {
        res = await fetch(args);
      } catch {
        throw Error(
          `An error during the attempt to fetch data from the server`
        );
      }

      if (!res.ok) {
        throw Error(`Error ${res.status} - ${res.statusText}`);
      }

      try {
        return res.json();
      } catch {
        throw Error(`Server response is not valid JSON`);
      }
    };
  }

  return useSWR(key, fetcher, {
    refreshInterval: 10e3,
    revalidateOnFocus: false,
    refreshWhenHidden: true,
  });
}
