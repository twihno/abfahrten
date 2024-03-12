import { default as Marquee } from "react-fast-marquee";
import {
  ErrorCircleFilled,
  InfoFilled,
  WarningFilled,
} from "@fluentui/react-icons";
import LiveClock from "./liveClock";
import { useSettings } from "../../hooks/useSettings";
import { CornerContent } from "../../types/settings";

export type marqueeMessage = {
  text: string;
  type: marqueeMessageType;
};

export enum marqueeMessageType {
  warning,
  info,
  error,
}

const iconClassNames = "mr-2" as const;

export function MarqueeFooter(props: {
  messages: marqueeMessage[];
}): JSX.Element {
  const settings = useSettings();

  const marqueeContent = props.messages.map((message: marqueeMessage) => {
    let icon: JSX.Element;
    switch (message.type) {
      case marqueeMessageType.warning:
        icon = <WarningFilled className={iconClassNames} fontSize={"2rem"} />;
        break;
      case marqueeMessageType.info:
        icon = <InfoFilled className={iconClassNames} fontSize={"2rem"} />;
        break;
      case marqueeMessageType.error:
        icon = (
          <ErrorCircleFilled className={iconClassNames} fontSize={"2rem"} />
        );
        break;
    }
    return (
      <div key={message.text} className="flex items-center mr-20">
        {icon}
        <span className="">{message.text} </span>
      </div>
    );
  });

  let cornerContentElement: JSX.Element | null = null;
  if (settings?.showCorner === "clock") {
    cornerContentElement = <LiveClock className="px-10 py-3 bg-slate-200" />;
  }

  return (
    <footer className="flex flex-row-reverse items-center bg-slate-100 h-12">
      {cornerContentElement}
      <Marquee gradient={false} speed={100}>
        {marqueeContent}
      </Marquee>
    </footer>
  );
}
