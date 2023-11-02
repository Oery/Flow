import Spinner from "./Spinner";
import Titlebar from "./TitleBar";

import styles from "../styles/LoadingScreen.module.css";
import LoginModal from "./LoginModal";
import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { Event, listen } from "@tauri-apps/api/event";

interface Props {
    isLoading: boolean;
}

export default function LoadingScreen({ isLoading }: Props) {
    const modal = useRef<HTMLDialogElement | null>(null);
    const [isLoggedIn, setIsLoggedIn] = useState(true);

    useEffect(() => {
        listen("logging-in", (event: Event<boolean>) => {
            console.log("Is Logging In Event fired", event.payload);
            setIsLoggedIn(event.payload);
        });
    }, []);

    useEffect(() => {
        setTimeout(() => {
            invoke("start_login_flow");
        }, 100);

        return () => {
            // todo cancel server
        };
    }, []);

    return (
        <div className={`${styles.loading} ${isLoading ? styles.end : ""}`}>
            <Titlebar />
            <h1>Flow</h1>
            <Spinner />
            <LoginModal ref={modal} isOpened={!isLoggedIn} />
        </div>
    );
}
