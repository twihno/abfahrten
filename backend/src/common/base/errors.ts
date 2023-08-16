import { log } from "./log.ts";

export class UserInputRequiredError extends Error {
  constructor(message?: string, options?: ErrorOptions) {
    super(message, options);
  }
}

export class BadRequestError extends Error {
  constructor(message?: string, options?: ErrorOptions) {
    super(message, options);
  }
}

export class NotFoundError extends Error {
  constructor(message?: string, options?: ErrorOptions) {
    super(message, options);
  }
}

export class InternalError extends Error {
  constructor(message?: string, options?: ErrorOptions) {
    super(message, options);
  }
}

/**
 * Prints an error based on an error object to the console
 * @param err
 */
export function printError(err: Error) {
  let message = err.message;

  if (message === "") {
    message = "Unknown error message";
  }

  log("ERROR", message);
}
