import { DateTime } from "luxon";
import { useEffect, useRef, useState } from "react";

type ClockProps = {
  twelveHours: boolean;
  secondsVisible: boolean;
  blinking: boolean;
  separator: string;
  className: string;
};

function generateText(date: DateTime, props: ClockProps): string {
  const separatorVisible = props.blinking ? date.second % 2 === 0 : true;
  const separatorFormatted = separatorVisible ? props.separator : " ";

  let timeString = "";
  let suffix = "";

  if (props.twelveHours) {
    timeString += (((date.hour + 11) % 12) + 1).toString().padStart(2, "0");
    if (date.hour >= 12) {
      suffix = " PM";
    } else {
      suffix = " AM";
    }
  } else {
    timeString += date.hour.toString().padStart(2, "0");
  }

  timeString += `${separatorFormatted}${date.minute
    .toString()
    .padStart(2, "0")}`;

  if (props.secondsVisible) {
    timeString += `${separatorFormatted}${date.second
      .toString()
      .padStart(2, "0")}`;
  }

  timeString += suffix;

  return timeString;
}

export default function LiveClock(props: Partial<ClockProps>): JSX.Element {
  const [, setUpdateTime] = useState<DateTime>(DateTime.now());
  const timeoutHandle = useRef<NodeJS.Timeout | undefined>(undefined);

  const completedProps: ClockProps = {
    twelveHours: false,
    secondsVisible: false,
    blinking: false,
    separator: ":",
    className: "",
    ...props,
  };

  useEffect(() => {
    if (timeoutHandle.current) {
      clearTimeout(timeoutHandle.current);
    }

    const currentTime = DateTime.now();

    const callback = () => {
      setUpdateTime(currentTime);
    };

    if (props.blinking || props.secondsVisible) {
      timeoutHandle.current = setTimeout(
        callback,
        1000 - currentTime.millisecond
      );
    } else {
      let delta = (60 - currentTime.second) * 1000 - currentTime.millisecond;
      if (delta > 1000) {
        delta -= 1000;
      }
      timeoutHandle.current = setTimeout(callback, delta);
    }

    return () => {
      clearTimeout(timeoutHandle.current);
    };
  });

  return (
    <span className={props.className}>
      {generateText(DateTime.now(), completedProps)}
    </span>
  );
}
