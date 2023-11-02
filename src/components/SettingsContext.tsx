import { createContext, useMemo, useState, useContext, useEffect } from "react";
import { updateTauriSetting } from "../utils/updateSetting";
import { invoke } from "@tauri-apps/api";

interface Settings {
    [key: string]: boolean | string;
}

interface SettingsContextType {
    settings: Settings;
    updateSetting: (key: string, value: any) => void;
}

export const SettingsContext = createContext<SettingsContextType | undefined>(
    undefined
);

export function useSettings(): SettingsContextType {
    const context = useContext(SettingsContext);
    if (!context) {
        throw new Error("useSettings must be used within a SettingsProvider");
    }
    return context;
}

interface SettingsProviderProps {
    children: React.ReactNode;
}

export const SettingsProvider: React.FC<SettingsProviderProps> = ({
    children,
}: any) => {
    const [settings, setSettings] = useState<Settings>({});

    const updateSetting = (key: string, value: any) => {
        updateTauriSetting(key, value);
        setSettings((prevSettings: Settings) => ({
            ...prevSettings,
            [key]: value,
        }));
    };

    useEffect(() => {
        console.log("Loading settings...");
        invoke<Settings>("load_settings")
            .then((appSettings) => {
                setSettings(appSettings);
                console.log(JSON.stringify(appSettings));
            })
            .catch((error: Error) => {
                console.error("Failed to load settings :", error);
            });
    }, []);

    const contextValue = useMemo(() => {
        return { settings, updateSetting };
    }, [settings]);

    return (
        <SettingsContext.Provider value={contextValue}>
            {children}
        </SettingsContext.Provider>
    );
};
