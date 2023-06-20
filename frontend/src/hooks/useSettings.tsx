"use client";

import { createContext, useContext, useEffect, useState } from "react";
import { ExtendedSettings, Settings } from "../types/settings";

export const SettingsContext = createContext<ExtendedSettings>(undefined);

export function useSettings() {
  return useContext(SettingsContext);
}

function getSettings(): Settings | null {
  const urlParams = new URLSearchParams(document.location.search);

  const urlSettings = urlParams.get("settings");
  if (!urlSettings) {
    return null;
  }

  let decodedSettings: string | undefined = undefined;
  try {
    decodedSettings = decodeURIComponent(urlSettings);
  } catch {
    return null;
  }

  const settings = JSON.parse(decodedSettings);

  return settings;
}

export function SettingsProvider(props: { children: JSX.Element }) {
  const [settings, setSettings] = useState<ExtendedSettings>(undefined);

  useEffect(() => {
    if (settings === undefined) {
      setSettings(getSettings());
    }
  }, [settings]);

  return (
    <SettingsContext.Provider value={settings}>
      {props.children}
    </SettingsContext.Provider>
  );
}
