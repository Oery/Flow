import { useState } from "react";
import { useAppContext } from "../AppContext";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/tauri";
import spinnerStyles from "../../styles/SpinnerSec.module.css";

export default function ObsStatus() {
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

    return (
        <div>
            <h3>OBS</h3>
            <p>
                {t("Status")} : {t(appContext.obs_status as string)}
                {appContext.obs_status === "Offline" && (
                    <span
                        onKeyDown={!loading ? connectToObs : undefined}
                        onClick={!loading ? connectToObs : undefined}
                        className={loading ? spinnerStyles.spinning : spinnerStyles.notspinning}
                    >
                        {" "}
                        ↺
                    </span>
                )}
            </p>
        </div>
    );
}
