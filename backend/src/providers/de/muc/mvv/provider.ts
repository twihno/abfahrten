import {
  Departure,
  Notification,
  VehicleType,
} from "../../../../common/departures.ts";
import { CompError, ErrorType } from "../../../../common/errors.ts";
import { getJSON } from "../../../../common/fetch.ts";
import { Station, StationIdList } from "../../../../common/stations.ts";
import {
  DepartureFinderResult,
  LineFinderResult,
  StopFinderResult,
} from "./api.ts";
import { base64Encode, DateTime } from "../../../../../deps.ts";
import { LineList } from "../../../../common/line.ts";
import { Provider as ProviderCommon } from "../../../../common/providers.ts";

function getVehicleType(name: string): VehicleType {
  switch (name) {
    case "U-Bahn":
      return VehicleType.U_BAHN;

    case "S-Bahn":
      return VehicleType.S_BAHN;

    case "Tram":
      return VehicleType.TRAM;

    case "Bus":
    case "MVV-Regionalbus":
      return VehicleType.BUS;

    case "ExpressBus":
      return VehicleType.EXPRESS_BUS;
  }
  return VehicleType.UNDEFINED;
}

function parsePlatformNumber(
  rawPlatform: string
): [platformNumber: string, label: string] {
  // Only number: e.g. S-Bahn
  if (rawPlatform.match(/^[1-9]+[0-9]*$/gm)) {
    return [rawPlatform, "Gleis"];
  }
  // U-Bahn
  if (rawPlatform.match(/^U-Bahn Gleis [1-9]+[0-9]*$/gm)) {
    return [rawPlatform.substring(13), "Gleis"];
  }
  if (rawPlatform.match(/^Gleis [1-9]+[0-9]*$/gm)) {
    return [rawPlatform.substring(6), "Gleis"];
  }
  // Tram
  if (rawPlatform.match(/^Pos. [1-9]+[0-9]*$/gm)) {
    return [rawPlatform.substring(5), "Pos."];
  }
  return ["X", "ERROR"];
}

/**
 * creates a request URL for the MVV API
 * @param stationId
 * @param lineIds list of all IDs for the requested lines
 * @param time unix timestamp representing the start of the requested interval
 * @returns url for the MVV API
 */
function getDepartureApiUrl(
  stationId: string,
  lineIds: string[],
  time: string
) {
  const baseUrl =
    "https://www.mvv-muenchen.de/?eID=departuresFinder&action=get_departures";
  const stopIdUrl = `stop_id=${encodeURIComponent(stationId)}`;
  const timestampURL = `requested_timestamp=${time}`;

  let linesURL = "";

  for (const lineId of lineIds) {
    linesURL += `&line=${encodeURIComponent(lineId)}`;
  }

  linesURL = base64Encode(linesURL);
  linesURL = encodeURIComponent(linesURL);
  linesURL = `lines=${linesURL}`;

  return [baseUrl, stopIdUrl, timestampURL, linesURL].join("&");
}
export class Provider implements ProviderCommon {
  name = "Münchner Verkehrsverbund";

  async findStationId(searchQuery: string): Promise<StationIdList> {
    let apiResponse: StopFinderResult;
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

  async getAvailableLines(stationId: string): Promise<LineList> {
    let apiResponse: LineFinderResult;
    try {
      apiResponse = await getJSON(
        `https://www.mvv-muenchen.de/?eID=departuresFinder&action=available_lines&stop_id=${stationId}`
      );
    } catch (e) {
      throw e;
    }

    if (apiResponse.error !== "") {
      throw new CompError(
        ErrorType.INTERNAL_ERROR,
        `internal error in MVV API; result: ${JSON.stringify(
          apiResponse.error
        )}`
      );
    }

    const availableLines: LineList = {};

    for (const line of apiResponse.lines) {
      availableLines[line.stateless] = {
        name: line.number,
        direction: line.direction,
        id: line.stateless,
        type: getVehicleType(line.name),
      };
    }
    return availableLines;
  }

  async getCurrentDepartures(
    station: Station,
    lines: LineList
  ): Promise<{
    departures: Departure[];
    notifications: Notification[];
  }> {
    const lineIds = Object.keys(lines);
    let apiResponse: DepartureFinderResult;

    const now = DateTime.now();
    try {
      apiResponse = await getJSON(
        getDepartureApiUrl(station.id, lineIds, now.toUnixInteger().toString())
      );
    } catch (e) {
      throw e;
    }

    if (apiResponse.error !== "") {
      throw new CompError(
        ErrorType.INTERNAL_ERROR,
        `internal error in MVV API; result: ${JSON.stringify(
          apiResponse.error
        )}`
      );
    }

    const notifications: Notification[] = apiResponse.notifications.map(
      (notification) => {
        return { text: notification.text, lines: [] };
      }
    );

    const departures: Departure[] = apiResponse.departures.map((departure) => {
      const lineId = departure.line.stateless;

      const [trackNumber, trackLabel] = parsePlatformNumber(departure.track);

      const departureDateString = `${departure.departureDate.substring(
        0,
        4
      )}-${departure.departureDate.substring(
        4,
        6
      )}-${departure.departureDate.substring(6, 8)}`;

      const departurePlanend = DateTime.fromISO(
        `${departureDateString}T${departure.departurePlanned}:00`,
        {
          zone: "Europe/Berlin",
        }
      );
      const departureLive = DateTime.fromISO(
        `${departureDateString}T${departure.departureLive}:00`,
        {
          zone: "Europe/Berlin",
        }
      );

      let notificationAvailable = false;
      if (departure.notifications.length !== 0) {
        for (const notification of departure.notifications) {
          let notificationExists = false;
          for (const existingNotification of notifications) {
            // Check if notification already exists
            if (notification.text === existingNotification.text) {
              notificationExists = true;
              // Check if line has already been added to the notification
              if (!existingNotification.lines.includes(lineId)) {
                existingNotification.lines.push(lineId);
              }
            }
          }
          if (!notificationExists) {
            notifications.push({
              text: notification.text,
              lines: [lineId],
            });
          }
        }
        notificationAvailable = true;
      }

      return {
        direction: departure.direction,
        lineId: lineId,
        trackLabel: trackLabel,
        track: trackNumber,
        departurePlanned: departurePlanend,
        departureLive: departureLive,
        onTime: departureLive.diff(departurePlanend).milliseconds === 0,
        notification: notificationAvailable,
        type: station.availableLines[lineId].type,
      };
    });

    return { departures: departures, notifications: Array.from(notifications) };
  }
}
