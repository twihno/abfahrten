import { CompError, ErrorType, exitError } from "./errors.ts";

type envVariables = {
  port: number;
  baseUrl: string;
  stationIdentifiers: string;
  useFirstResult: boolean;
  fetchInterval: number;
  cors: string;
};

function parseAndValidateInt(q: string, variableName: string) {
  const numberString = q?.trim();

  if (numberString.match("^0+$") || !numberString.match("^[0-9]+$")) {
    throw new CompError(
      ErrorType.BAD_REQUEST,
      `invalid value for "${variableName}"`
    );
  }

  return Number.parseInt(numberString);
}

export function getEnv(): envVariables {
  const env: envVariables = {
    port: 8080,
    baseUrl: "/api",
    stationIdentifiers: "",
    useFirstResult: true,
    fetchInterval: 30,
    cors: "*",
  };

  const envStationIdentifiers = Deno.env.get("STATIONS");

  if (envStationIdentifiers === undefined || !envStationIdentifiers.trim()) {
    exitError(new CompError(ErrorType.BAD_REQUEST, "no stations defined"));
  }

  env.stationIdentifiers = envStationIdentifiers;

  const envUseFirstResult = Deno.env.get("USE_FIRST_RESULT")?.toLowerCase();

  if (envUseFirstResult !== undefined) {
    if (!["true", "false"].includes(envUseFirstResult)) {
      exitError({
        type: ErrorType.BAD_REQUEST,
        msg: 'invalid value for "USE_FIRST_RESULT"',
      });
    }
    env.useFirstResult = envUseFirstResult === "true";
  }

  const envFetchInterval = Deno.env.get("FETCH_INTERVAL");

  if (envFetchInterval !== undefined) {
    try {
      env.fetchInterval = parseAndValidateInt(
        envFetchInterval,
        "FETCH_INTERVAL"
      );
    } catch (e) {
      exitError(e);
    }
  }

  const envPort = Deno.env.get("PORT");
  if (envPort !== undefined) {
    try {
      env.port = parseAndValidateInt(envPort, "PORT");
    } catch (e) {
      exitError(e);
    }
  }

  const envBaseUrl = Deno.env.get("BASE_URL");
  if (envBaseUrl !== undefined) {
    env.baseUrl = envBaseUrl;
  }

  const envCors = Deno.env.get("CORS");
  if (envCors !== undefined) {
    env.cors = envCors;
  }

  return env;
}
