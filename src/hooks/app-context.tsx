import type { Context as App } from '@/types/Context';

import { createContext, useState, useContext, useEffect } from 'react';
import { type Event, listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

export const AppContext = createContext<App | undefined>(undefined);

export function useAppContext(): App {
    const context = useContext(AppContext);
    if (!context) throw new Error('useApp must be used within a AppProvider');
    return context;
}

export const AppProvider = ({ children }: React.PropsWithChildren) => {
    const [context, setContext] = useState<App>({
        streamer: { color: '', id: '', display_name: '', avatar_url: '', emotes: [] },
        resource_packs_raw: [],
        aliases: {
            packs: {},
            servers: {},
        },
        custom_bot_id: '',
        client: '',
        event_loop_running: false,
        ingame_status: 'Unknown',
        bot_status: 'Offline',
        obs_status: 'Offline',
        song_title: '?',
        song_author: '?',
        resource_pack_str: '?',
        server_address: '?',
        server_raw: '?',
        twitch_access_token: '',
        custom_bot_token: '',
    });

    useEffect(() => {
        invoke<App>('load_context').then((appContext) => {
            setContext(appContext);
        });

        const unlisten = listen('update-context', (event: Event<App>) => {
            setContext((currentContext) => ({
                ...currentContext,
                ...event.payload,
            }));
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    return <AppContext.Provider value={context}>{children}</AppContext.Provider>;
};
