"use client";

import { useSettings } from "./settings";

export default function Departures() {
  const settings = useSettings();

  if (settings === undefined) {
    return <div>Loading...</div>;
  }
  if (settings === null) {
    return <div>Error: Invalid settings</div>;
  }

  return <div className="w-full h-full">Liste!!!!</div>;
}
