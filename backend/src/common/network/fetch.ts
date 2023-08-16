import { CompError, ErrorType } from "./errors.ts";

export async function getJSON(url: string) {
  let response;
  try {
    response = await fetch(url);
  } catch (e) {
    throw e;
  }

  if (!response.ok) {
    throw new CompError(
      ErrorType.INTERNAL_ERROR,
      `GET URL: "${url}"; ${response.status}: ${response.statusText}`
    );
  }

  return response.json();
}
