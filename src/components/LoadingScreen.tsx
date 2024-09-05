import Spinner from "./ui/spinner";
import Titlebar from "./title-bar";

import styles from "../styles/LoadingScreen.module.css";
import LoginModal from "./LoginModal";
import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { type Event, listen } from "@tauri-apps/api/event";

interface Props {
    isLoading: boolean;
}

export default function LoadingScreen({ isLoading }: Readonly<Props>) {
    const modal = useRef<HTMLDialogElement | null>(null);
    const [isLoggedIn, setIsLoggedIn] = useState(true);

    useEffect(() => {
        const unlisten = listen("logging-in", (event: Event<boolean>) => {
            setIsLoggedIn(event.payload);
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    useEffect(() => {
        setTimeout(() => {
            invoke("load_app");
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
