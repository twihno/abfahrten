import Departures from "../../components/common/departures";
import { SettingsProvider } from "../../hooks/useSettings";

export default function Home() {
  return (
    <SettingsProvider>
      <div className="w-full h-full flex justify-center items-center flex-col text-xl">
        <Departures />
      </div>
    </SettingsProvider>
  );
}
