export type stopFinderResult = {
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
