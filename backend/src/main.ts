import { version } from "../deps.ts";
import { loadEnv } from "./common/base/env.ts";
import { initExitCleanup } from "./common/base/exit.ts";
import { initLogging, log } from "./common/base/log.ts";

initLogging();
initExitCleanup();

log("INFO", `Abfahrten server - version ${version}`);

loadEnv();
