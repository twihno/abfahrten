export type DisplayType = "list" | "screen";

export type Settings = {
  serverURL: string;
  station: string;
  displayType: DisplayType;
};

export type ExtendedSettings = Settings | null | undefined;
