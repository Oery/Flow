import { useAppContext } from "../AppContext";
import styles from "../../styles/AsideInfo.module.css";
import BotStatus from "./bot-status";

import ClientStatus from "./client-status";
import ObsStatus from "./obs-status";

export default function AsideInfo() {
    const appContext = useAppContext();

    console.log(appContext);

    return (
        <aside className={styles.aside}>
            <header>
                <h1>{appContext.streamer.display_name}</h1>
            </header>

            <div>
                <ObsStatus />
                <BotStatus />
                <ClientStatus />
            </div>
        </aside>
    );
}
