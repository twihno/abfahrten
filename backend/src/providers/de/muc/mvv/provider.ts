import { CompError, ErrorType } from "../../../../common/errors.ts";
import { getJSON } from "../../../../common/fetch.ts";
import { StationIdList } from "../../../../common/stations.ts";
import { stopFinderResult } from "./api.ts";

export const name = "Münchner Verkehrsverbund";

export async function findStationId(
  searchQuery: string
): Promise<StationIdList> {
  let apiResponse: stopFinderResult;
  try {
    apiResponse = await getJSON(
      `https://www.mvv-muenchen.de/?eID=stopFinder&query=${searchQuery}`
    );
  } catch (e) {
    throw e;
  }

  if (!apiResponse.success) {
    throw new CompError(
      ErrorType.INTERNAL_ERROR,
      `internal error in MVV API; result: ${JSON.stringify(
        apiResponse.message
      )}`
    );
  }

  if (apiResponse.results.length === 0) {
    return [];
  }

  const stops = apiResponse.results.filter((res) => res.anyType === "stop");

  return stops.map((res) => {
    return {
      name: res.name,
      id: res.ref.gid,
    };
  });
}
