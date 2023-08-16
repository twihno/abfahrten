import { log } from "./log.ts";

export let oakAbortController: AbortController;
let oakApplicationPromise: Promise<void>;

const gracefulSignals: Deno.Signal[] = [
  "SIGINT",
  "SIGTERM",
  "SIGBREAK",
  "SIGQUIT",
];

/**
 * Stop running processes and exit the application
 */
async function handleGracefulExit() {
  log("INFO", "Stopping service");

  oakAbortController.abort();
  await oakApplicationPromise;

  Deno.exit(0);
}

/**
 * Init methods for handling a clean exit
 */
export function initExitCleanup() {
  oakAbortController = new AbortController();

  for (const signal of gracefulSignals) {
    try {
      Deno.addSignalListener(signal, handleGracefulExit);
    } catch {
      continue;
    }
  }
}

/**
 *
 * @param oakApplicationCall Promise returned by
 */
export function bindOakApplication(oakApplicationCall: Promise<void>) {
  oakApplicationPromise = oakApplicationCall;
}

/**
 * Logs an error terminates the program
 * @param err
 */
export function exitWithError(err: Error): never {
  let message = err.message;

  if (message === "") {
    message = "Unknown error message";
  }

  log("CRITICAL", message);
  Deno.exit(1);
}
