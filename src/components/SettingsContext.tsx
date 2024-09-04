import { createContext, useMemo, useState, useContext, useEffect } from "react";
import { removeTauriAlias, updateTauriAlias, updateTauriSetting } from "../utils/updateSetting";
import { invoke } from "@tauri-apps/api";
import { useTranslation } from "react-i18next";
import type { AliasGroup, Aliases } from "../types/Alias";

type Setting = boolean | string;

interface AliasStore {
    packs: Aliases;
    servers: Aliases;
}

interface Settings {
    [key: string]: Setting | AliasStore;
    aliases: AliasStore;
}

interface SettingsContextType {
    settings: Settings;
    updateSetting: (key: string, value: Setting) => void;
    updateAlias: (group: AliasGroup, key: string, value: string) => void;
    removeAlias: (group: AliasGroup, name: string) => void;
}

export const SettingsContext = createContext<SettingsContextType | undefined>(undefined);

export function useSettings(): SettingsContextType {
    const context = useContext(SettingsContext);
    if (!context) {
        throw new Error("useSettings must be used within a SettingsProvider");
    }
    return context;
}

export const SettingsProvider = ({ children }: React.PropsWithChildren) => {
    const [settings, setSettings] = useState<Settings>({
        aliases: {
            packs: {},
            servers: {},
        },
    });
    const { i18n } = useTranslation();

    const updateSetting = (key: string, value: Setting) => {
        updateTauriSetting(key, value);
        setSettings((prevSettings: Settings) => ({
            ...prevSettings,
            [key]: value,
        }));
    };

    const updateAlias = (group: AliasGroup, key: string, value: string) => {
        updateTauriAlias(group, key, value);

        setSettings((prevSettings: Settings) => {
            const store = prevSettings.aliases[group];

            if (store[key]) store[key].alias = value;
            else store[key] = { alias: value, hidden: false };

            return {
                ...prevSettings,
                aliases: { ...prevSettings.aliases, [group]: store },
            };
        });
    };

    const removeAlias = (group: AliasGroup, name: string) => {
        removeTauriAlias(group, name);

        setSettings((prevSettings: Settings) => {
            const store = prevSettings.aliases[group];
            delete store[name];

            return {
                ...prevSettings,
                aliases: { ...prevSettings.aliases, [group]: store },
            };
        });
    };

    useEffect(() => {
        console.log("Loading settings...");
        invoke<Settings>("load_settings")
            .then((appSettings) => {
                setSettings(appSettings);

                if (appSettings.language !== "auto") {
                    console.log("Language set to : ", appSettings.language);
                    i18n.changeLanguage(appSettings.language as string);
                }
            })
            .catch((error: Error) => {
                console.error("Failed to load settings :", error);
            });
    }, [i18n.changeLanguage]);

    const contextValue = useMemo(() => {
        return { settings, updateSetting, updateAlias, removeAlias };
    }, [settings, removeAlias, updateAlias, updateSetting]);

    return <SettingsContext.Provider value={contextValue}>{children}</SettingsContext.Provider>;
};
