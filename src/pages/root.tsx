import { Outlet } from "react-router-dom";
import NavElement from "../components/ui/nav-element";
import { type Event, listen } from "@tauri-apps/api/event";

import styles from "../styles/Root.module.css";
import Titlebar from "../components/title-bar";

import { SettingsProvider } from "../components/SettingsContext";
import { AppProvider } from "../components/AppContext";
import { useEffect, useState } from "react";
import LoadingScreen from "../components/LoadingScreen";
import AsideInfo from "../components/aside/aside-infos";
import { useTranslation } from "react-i18next";
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";

export default function Root() {
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const [stopLoader, setStopLoader] = useState<boolean>(false);
    const { t } = useTranslation();

    useEffect(() => {
        const unlisten = listen("loading-end", (event: Event<boolean>) => {
            if (event.payload) {
                setStopLoader(true);
                invoke("start_event_loop");
                setTimeout(() => {
                    setIsLoading(false);
                }, 500);
            }
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    return (
        <main className='bg-flow-secondary w-screen'>
            <title>Flow</title>
            <SettingsProvider>
                <AppProvider>
                    {isLoading ? (
                        <LoadingScreen isLoading={stopLoader} />
                    ) : (
                        <>
                            <Titlebar />

                            <div className={styles.maincontainer}>
                                <main className=' bg-flow-secondary'>
                                    <Outlet />
                                </main>
                                <AsideInfo />
                            </div>
                        </>
                    )}
                </AppProvider>
            </SettingsProvider>
        </main>
    );
}
