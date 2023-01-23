import con from "./console.ts";

/**
 * Available error types for CompErrors
 */
export enum ErrorType {
  UNDEFINED = 1,
  USER_INPUT_REQUIRED = 1,
  BAD_REQUEST = 400,
  NOT_FOUND = 404,
  INTERNAL_ERROR = 500,
}

/**
 * Composite error made up of a type and a (optional) message
 */
export class CompError {
  type: ErrorType;
  msg?: string;

  constructor(type: ErrorType, msg?: string) {
    (this.type = type), (this.msg = msg);
  }
}

/**
 * Prints an error based on an error object to the console
 * @param err
 */
export function printError(err: Error | CompError) {
  if ("type" in err) {
    con.error(err.msg ? err.msg : err.type.toString());
  } else {
    con.error(err.message);
  }
}

/**
 * Prints the error to the console and terminates the program
 * @param err
 */
export function exitError(err: Error | CompError): never {
  let code = 1;
  let message = "";
  if ("type" in err) {
    if (err.msg) {
      message = `${err.msg}; Code: ${err.type}`;
    } else {
      message = err.type.toString();
    }
    code = err.type;
  } else {
    message = err.message;
  }

  console.error(
    `%cfatal error%c:%c ${message}`,
    "font-weight: bold; color: red;",
    "color: red;",
    ""
  );
  Deno.exit(code);
}
