"use client";

import { useDepartures } from "../../hooks/useDepartures";
import { useSettings } from "../../hooks/useSettings";

export default function Departures() {
  const settings = useSettings();
  const { data: departures, error, isLoading } = useDepartures(settings);

  if (settings === undefined) {
    return <div>Loading...</div>;
  }
  if (settings === null) {
    return <div>Error: Invalid settings</div>;
  }

  console.log(departures);

  return <div className="w-full h-full">Liste!!!!</div>;
}
