import { createContext, useState, useContext, useEffect } from "react";
import { Event, listen } from "@tauri-apps/api/event";
import { App } from "../types/Context";
import { invoke } from "@tauri-apps/api/tauri";

export const AppContext = createContext<App | undefined>(undefined);

export function useAppContext(): App {
    const context = useContext(AppContext);
    if (!context) throw new Error("useApp must be used within a AppProvider");
    return context;
}

export const AppProvider = ({ children }: React.PropsWithChildren) => {
    const [context, setContext] = useState<App>({
        streamer: { color: "", id: "", display_name: "", avatar_url: "" },
        resource_packs_raw: [],
        aliases: {
            packs: {},
            servers: {},
        },
    });

    useEffect(() => {
        invoke<App>("load_context").then((appContext) => {
            setContext(appContext);
        });

        const unlisten = listen("update-context", (event: Event<App>) => {
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
