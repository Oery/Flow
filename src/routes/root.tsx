import { Outlet } from "react-router-dom";
import NavElement from "../components/NavElement";
import { Event, listen } from "@tauri-apps/api/event";

import styles from "../styles/Root.module.css";
import Titlebar from "../components/TitleBar";

import { SettingsProvider } from "../components/SettingsContext";
import { AppProvider } from "../components/AppContext";
import { appWindow } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";
import LoadingScreen from "../components/LoadingScreen";
import AsideInfo from "../components/AsideInfos";

export default function Root() {
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const [stopLoader, setStopLoader] = useState<boolean>(false);

    useEffect(() => {
        listen("loading-end", (event: Event<boolean>) => {
            console.log(event.payload);
            if (event.payload) {
                console.log("Payload received");
                setStopLoader(true);
                setTimeout(() => {
                    setIsLoading(false);
                }, 500);
            }
        });
    }, []);

    useEffect(() => {
        setTimeout(() => {
            appWindow.show();
        }, 50);

        console.log("function ran");
        // Check Updates
        // Should show Initial Config ?
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
                                                <NavElement
                                                    title="Home"
                                                    link=""
                                                />
                                                <NavElement
                                                    title="Scene"
                                                    link="scene"
                                                />
                                                <NavElement
                                                    title="Pack"
                                                    link="dynpack"
                                                />
                                                <NavElement
                                                    title="Server"
                                                    link="dynip"
                                                />
                                                <NavElement
                                                    title="Music"
                                                    link="music"
                                                />
                                            </div>

                                            <footer>
                                                <NavElement
                                                    title="Settings"
                                                    link="settings"
                                                />
                                                <NavElement
                                                    title="Log out"
                                                    link="log-out"
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
