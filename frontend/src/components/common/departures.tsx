"use client";

import { useDepartures } from "../../hooks/useDepartures";
import { useSettings } from "../../hooks/useSettings";

export default function Departures() {
  const settings = useSettings();
  const { data: departures, error, isLoading } = useDepartures(settings);

  if (settings === undefined || isLoading) {
    return <div>Loading...</div>;
  }
  if (settings === null) {
    return <div>Error: Invalid settings</div>;
  }

  if (error) {
    return <div>Error in data fetching: {error.message}</div>;
  }

  console.log(departures);

  return <div className="w-full h-full">Liste!!!!</div>;
}
