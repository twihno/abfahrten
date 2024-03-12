"use client";

import { useDepartures } from "../../hooks/useDepartures";
import { useSettings } from "../../hooks/useSettings";
import { MarqueeFooter, marqueeMessageType } from "./marqueeFooter";

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

  if (!departures) {
    return <div>No departure information available</div>;
  }

  console.log(departures);

  let footer = null;
  if (departures.notifications.length !== 0) {
    footer = (
      <MarqueeFooter
        messages={departures.notifications.map((notification) => {
          return { text: notification.text, type: marqueeMessageType.warning };
        })}
      />
    );
  }

  return (
    <div className="grid grid-rows-[minmax(0,1fr)_min-content] h-full w-full">
      <div>Liste</div>
      {footer}
    </div>
  );
}
