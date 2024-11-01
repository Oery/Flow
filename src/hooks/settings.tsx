import { removeTauriAlias, updateTauriAlias, updateTauriSetting } from '@/utils/update-settings';
import type { Settings } from '@/types/Settings';
import type { Aliases } from '@/types/Aliases';

import { createContext, useMemo, useState, useContext, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api';
import { useTranslation } from 'react-i18next';

interface SettingsContextType {
    settings: Settings;
    updateSetting: (key: string, value: unknown) => void;
    updateAlias: (group: keyof Aliases, key: string, value: string) => void;
    removeAlias: (group: keyof Aliases, name: string) => void;
}

export const SettingsContext = createContext<SettingsContextType | undefined>(undefined);

export function useSettings(): SettingsContextType {
    const context = useContext(SettingsContext);
    if (!context) {
        throw new Error('useSettings must be used within a SettingsProvider');
    }
    return context;
}

export const SettingsProvider = ({ children }: React.PropsWithChildren) => {
    const [settings, setSettings] = useState<Settings>({
        aliases: {
            packs: {},
            servers: {},
        },
        language: '',
        custom_logs_path: '',
        twitch_bot: '',
        bot_prefix: '',
        enable: true,
        streaming_only: true,
        streaming_mc_only: true,
        mc_client: 'Vanilla / Forge',
        custom_bot_id: '',
        disable_hardware_acceleration: false,
        start_with_windows: false,
        scenes_enable: false,
        scenes_name: 'Queuing Scene',
        scenes_delay: 3,
        scenes_hide_in_lobby: false,
        scenes_toggle_after_game_end: true,
        scenes_auto_obs_config: true,
        scenes_obs_ws_port: 4455,
        scenes_obs_ws_password: '',
        scenes_obs_config_path: '%APPDATA%\\obs-studio',
        pack_enable: false,
        pack_command: '!pack',
        pack_command_text: 'Resource Pack : {pack}',
        pack_hide_overlay: false,
        pack_announcements_enable: true,
        pack_announcements_text: 'Resource Pack : {pack}',
        server_enable: false,
        server_command: '!ip',
        server_command_text: 'IP : {server}',
        server_announcements_enable: true,
        server_announcements_text: 'IP : {server}',
        music_enable: false,
        music_ignore_twitch: true,
        music_command: '!music',
        music_command_text: '🎵 : {title} - {artist}',
        music_announcements_enable: true,
        music_announce_text: '🎵 Now playing : {title} - {artist} 🎵',
    });
    const { i18n } = useTranslation();

    const updateSetting = useCallback((key: string, value: unknown) => {
        updateTauriSetting(key, value);
        setSettings((prevSettings: Settings) => ({
            ...prevSettings,
            [key]: value,
        }));
    }, []);

    const updateAlias = useCallback((group: keyof Aliases, key: string, value: string) => {
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
    }, []);

    const removeAlias = useCallback((group: keyof Aliases, name: string) => {
        removeTauriAlias(group, name);

        setSettings((prevSettings: Settings) => {
            const store = prevSettings.aliases[group];
            delete store[name];

            return {
                ...prevSettings,
                aliases: { ...prevSettings.aliases, [group]: store },
            };
        });
    }, []);

    useEffect(() => {
        console.log('Loading settings...');
        invoke<Settings>('load_settings')
            .then((appSettings) => {
                setSettings(appSettings);

                if (appSettings.language !== 'auto') {
                    console.log('Language set to : ', appSettings.language);
                    i18n.changeLanguage(appSettings.language);
                }
            })
            .catch((error: Error) => {
                console.error('Failed to load settings :', error);
            });
    }, [i18n.changeLanguage]);

    const contextValue = useMemo(() => {
        return { settings, updateSetting, updateAlias, removeAlias };
    }, [settings, removeAlias, updateAlias, updateSetting]);

    return <SettingsContext.Provider value={contextValue}>{children}</SettingsContext.Provider>;
};
