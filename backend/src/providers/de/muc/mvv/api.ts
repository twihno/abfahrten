export type StopFinderResult = {
  success: boolean;
  message: string;
  results: {
    usage: string;
    type: string;
    name: string;
    stateless: string;
    anyType: string; // primarily "stop" or "poi"
    sort: string;
    quality: string;
    best: string;
    object: string;
    mainLoc: string;
    modes: string;
    ref: {
      id: string;
      gid: string;
      omc: string;
      placeID: string;
      place: string;
      coords: string;
    };
    infos: null; // maybe different, but currently irrelevant
    hasStaticInfo: string;
  }[];
};

export type ErrorResult = {
  error: string;
};

export type MvvNotification = {
  text: string;
  link: string;
  type: string;
};

export type MvvLine = {
  number: string;
  symbol: string;
  direction: string;
  stateless: string;
  name: string;
};

export type Departure = {
  line: MvvLine;
  direction: string;
  station: {
    id: string;
    name: string;
  };
  track: string;
  departureDate: string;
  departurePlanned: string;
  departureLive: string;
  inTime: boolean;
  notifications: MvvNotification[];
};

export type DepartureFinderResult = {
  error: string;
  departures: Departure[];
  notifications: MvvNotification[];
};

export type LineFinderResult = {
  error: string;
  lines: MvvLine[];
  icons: []; // currently unknown but irrelevant
};
