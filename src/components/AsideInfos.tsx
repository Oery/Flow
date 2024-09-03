import { useAppContext } from "./AppContext";
import styles from "../styles/AsideInfo.module.css";
import BotStatus from "./aside/bot-status";
import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import { useTranslation } from "react-i18next";

import spinnerStyles from "../styles/SpinnerSec.module.css";
import ClientStatus from "./aside/client-status";

export default function AsideInfo() {
    const appContext = useAppContext();
    const [loading, setLoading] = useState<boolean>(false);
    const { t } = useTranslation();

    const connectToObs = () => {
        setLoading(true);
        invoke("connect_to_obs")
            .then(() => {
                setLoading(false);
                appContext.obs_status = "Online";
            })
            .catch(() => setLoading(false));
    };

    console.log(appContext);

    return (
        <aside className={styles.aside}>
            <header>
                <h1>{appContext.streamer.display_name}</h1>
            </header>

            <div>
                <div>
                    <h3>OBS</h3>
                    <p>
                        {t("Status")} : {t(appContext.obs_status as string)}
                        {appContext.obs_status === "Offline" && (
                            <span
                                onKeyDown={!loading ? connectToObs : undefined}
                                onClick={!loading ? connectToObs : undefined}
                                className={
                                    loading ? spinnerStyles.spinning : spinnerStyles.notspinning
                                }
                            >
                                {" "}
                                ↺
                            </span>
                        )}
                    </p>
                </div>

                <BotStatus />
                <ClientStatus />
            </div>
        </aside>
    );
}
