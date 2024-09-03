import { Outlet } from "react-router-dom";
import NavElement from "../components/ui/nav-element";
import { Event, listen } from "@tauri-apps/api/event";

import styles from "../styles/Root.module.css";
import Titlebar from "../components/title-bar";

import { SettingsProvider } from "../components/SettingsContext";
import { AppProvider } from "../components/AppContext";
import { useEffect, useState } from "react";
import LoadingScreen from "../components/LoadingScreen";
import AsideInfo from "../components/AsideInfos";
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
        <>
            <title>Flow</title>
            <SettingsProvider>
                <AppProvider>
                    {isLoading ? (
                        <LoadingScreen isLoading={stopLoader} />
                    ) : (
                        <>
                            <Titlebar />

                            <div className={styles.maincontainer}>
                                <div className={styles.sidebar}>
                                    <header>
                                        <h1>Flow</h1>
                                    </header>
                                    <nav>
                                        <ul>
                                            <div>
                                                <NavElement title={t("Home")} link="" />
                                                <NavElement title={t("Scene")} link="scene" />
                                                <NavElement title={t("Packs")} link="dynpack" />
                                                <NavElement title={t("Server")} link="dynip" />
                                                <NavElement title={t("Music")} link="music" />
                                                <NavElement title={t("Bot")} link="bot" />
                                                {/* <NavElement
                                                    title="Stats"
                                                    link="stats"
                                                /> */}
                                            </div>

                                            <footer>
                                                <NavElement title={t("Settings")} link="settings" />
                                                <NavElement
                                                    title={t("Exit")}
                                                    link="log-out"
                                                    handleClick={() => {
                                                        appWindow.close();
                                                    }}
                                                />
                                            </footer>
                                        </ul>
                                    </nav>
                                </div>

                                <main>
                                    {/* <div>{JSON.stringify(settings)}</div> */}
                                    {/* <button onClick={handleClick}>LOAD SETTINGS</button> */}

                                    <Outlet />
                                </main>

                                <AsideInfo />
                            </div>
                        </>
                    )}
                </AppProvider>
            </SettingsProvider>
        </>
    );
}
