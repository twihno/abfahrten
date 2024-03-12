export type DisplayType = "list" | "screen";

export type Settings = {
  serverURL: string;
  station: string;
  displayType: DisplayType;
  showCorner: CornerContent;
};

export type ExtendedSettings = Settings | null | undefined;
export type CornerContent = null | "clock" | "date";
