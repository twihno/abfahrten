import { Logger, LevelName, ConsoleHandler } from "../../../deps.ts";

let logger: Logger;

/**
 * Initializes the logging
 * (This has to be the first thing to be called in main.ts!!!)
 */
export function initLogging() {
  const envLogLevel = Deno.env.get("LOG_LEVEL");

  let logLevel: LevelName = "INFO";
  if (envLogLevel) {
    if (["DEBUG", "INFO", "WARNING"].includes(envLogLevel)) {
      logLevel = envLogLevel as LevelName;
    } else {
      // Can't use exitWithError, because logging isn't initialized yet...
      console.error(
        `%cCRITICAL: Invalid LOG_LEVEL "${envLogLevel}"." Allowed values: "DEBUG", "INFO", "WARNING"`,
        "font-weight: bold; color: red;"
      );
      Deno.exit(1);
    }
  }

  logger = new Logger("logger", logLevel, {
    handlers: [
      new ConsoleHandler(logLevel, {
        formatter: "{levelName}: {msg}",
      }),
    ],
  });
}

/**
 * logs a message at a provided level
 * @param level log level
 * @param msg message
 */
export function log(level: LevelName, msg: string) {
  switch (level) {
    case "DEBUG":
      logger.debug(msg);
      break;
    case "INFO":
      logger.info(msg);
      break;
    case "WARNING":
      logger.warning(msg);
      break;
    case "ERROR":
      logger.error(msg);
      break;
    case "CRITICAL":
      logger.critical(msg);
      break;
    default:
      logger.info(msg);
      break;
  }
}
