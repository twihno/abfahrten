export default class con {
  static error(msg: string): void {
    console.error(
      `%cerror%c:%c ${msg}`,
      "font-weight: bold; color: red;",
      "color: red;",
      ""
    );
  }

  static info(msg: string): void {
    console.info(
      `%cinfo%c:%c ${msg}`,
      "font-weight: bold; color: blue;",
      "color: blue;",
      ""
    );
  }

  static warn(msg: string): void {
    console.warn(
      `%cwarning%c:%c ${msg}`,
      "font-weight: bold; color: yellow;",
      "color: blue;",
      ""
    );
  }
}
