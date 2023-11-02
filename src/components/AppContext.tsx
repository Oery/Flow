import { createContext, useState, useContext, useEffect } from "react";
import { Event, listen } from "@tauri-apps/api/event";

interface Streamer {
    color: string;
    id: string;
    display_name: string;
    avatar_url: string;
}

interface App {
    [key: string]: boolean | string | Streamer;
    streamer: Streamer;
}

export const AppContext = createContext<App | undefined>(undefined);

export function useAppContext(): App {
    const context = useContext(AppContext);
    if (!context) {
        throw new Error("useApp must be used within a AppProvider");
    }
    return context;
}

interface AppProviderProps {
    children: React.ReactNode;
}

export const AppProvider: React.FC<AppProviderProps> = ({ children }: any) => {
    const [context, setContext] = useState<App>({
        streamer: { color: "", id: "", display_name: "", avatar_url: "" },
    });

    useEffect(() => {
        listen("update-context", (event: Event<App>) => {
            console.log("event fire", event.payload);
            const newContextData = event.payload;
            setContext(newContextData);
        });
    }, []);

    return (
        <AppContext.Provider value={context}>{children}</AppContext.Provider>
    );
};
