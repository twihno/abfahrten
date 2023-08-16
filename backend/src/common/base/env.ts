import { BadRequestError } from "./errors.ts";
import { exitWithError } from "./exit.ts";

type envVariables = {
  stationIdentifiers: string;
  useFirstResult: boolean;
  fetchInterval: number;
  port: number;
  baseUrl: string;
  cors: string | null;
};

function parseAndValidateInt(q: string, variableName: string) {
  const numberString = q.trim();

  if (numberString.match("^0+$") || !numberString.match("^[0-9]+$")) {
    throw new BadRequestError(`invalid value for "${variableName}"`);
  }

  return Number.parseInt(numberString);
}

export const env: envVariables = {
  stationIdentifiers: "",
  useFirstResult: true,
  fetchInterval: 30,
  port: 8080,
  baseUrl: "/api",
  cors: null,
};

/**
 * Loads and validates the environment variables
 * Exits the program if a critical error is encountered
 */
export function loadEnv(): void {
  // STATIONS (required)
  // Which stations should be available
  // List separated by `;`
  const envStationIdentifiers = Deno.env.get("STATIONS");
  if (envStationIdentifiers === undefined || !envStationIdentifiers.trim()) {
    exitWithError(new BadRequestError("no stations defined"));
  }
  env.stationIdentifiers = envStationIdentifiers;

  // USE_FIRST_RESULT (optional, default: true)
  // Use the first result if a search is necessary
  // "true" or "false"
  const envUseFirstResult = Deno.env.get("USE_FIRST_RESULT");
  if (envUseFirstResult !== undefined) {
    if (!["true", "false"].includes(envUseFirstResult.toLowerCase())) {
      exitWithError(
        new BadRequestError('invalid value for "USE_FIRST_RESULT"')
      );
    }
    env.useFirstResult = envUseFirstResult === "true";
  }

  // FETCH_INTERVAL (optional, default: 30 (seconds))
  // Seconds between fetches
  // Positive integer
  const envFetchInterval = Deno.env.get("FETCH_INTERVAL");
  if (envFetchInterval !== undefined) {
    try {
      env.fetchInterval = parseAndValidateInt(
        envFetchInterval,
        "FETCH_INTERVAL"
      );
    } catch (e) {
      exitWithError(e);
    }
  }

  // PORT (optional, default: 8080)
  // Which port the program should listen on
  // Positive integer (and also a valid port number)
  const envPort = Deno.env.get("PORT");
  if (envPort !== undefined) {
    try {
      env.port = parseAndValidateInt(envPort, "PORT");
    } catch (e) {
      exitWithError(e);
    }
  }

  // BASE_URL (optional, default: /api)
  // Base URL for the api server
  // /<some_url>
  const envBaseUrl = Deno.env.get("BASE_URL");
  if (envBaseUrl !== undefined) {
    env.baseUrl = envBaseUrl.trim();

    // Leading "/"
    if (env.baseUrl.charAt(0) !== "/") {
      env.baseUrl = `/${env.baseUrl}`;
    }

    // No trailing "/"
    if (env.baseUrl.charAt(env.baseUrl.length - 1) === "/") {
      env.baseUrl = env.baseUrl.substring(0, env.baseUrl.length - 1);
    }
  }

  // CORS (optional: default: null)
  // Value for the `Access-Control-Allow-Origin` header
  // String value for the header; if no value is provided, the header will be omitted
  const envCors = Deno.env.get("CORS");
  if (envCors !== undefined) {
    env.cors = envCors;
  }

  // Handled in log.ts (logging has to be initialized as the first (non-main) module):
  // LOG_LEVEL (optional, default: INFO)
  // Min level for logging
  // DEBUG | INFO | WARNING

  Object.freeze(env);
}
