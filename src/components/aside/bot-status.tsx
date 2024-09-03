import { useSettings } from "../SettingsContext";
import { useTranslation } from "react-i18next";
import { useAppContext } from "../AppContext";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api";

const botDisplayNames: Record<string, string> = {
    self: "Self",
    custom: "Custom",
    nightbot: "Nightbot",
    wizebot: "WizeBot",
};

export default function BotStatus() {
    const { settings } = useSettings();
    const appContext = useAppContext();
    const { t } = useTranslation();

    useEffect(() => {
        invoke("set_current_bot", { bot: settings.twitch_bot });
    }, [settings.twitch_bot]);

    return (
        <div>
            <h3>{botDisplayNames[settings.twitch_bot as string]}</h3>
            {/* Ajouter un bouton rouge ! pour ouvrir le modal de Nightbot en cas de token invalide */}
            <p>
                {t("Status")} : {t(appContext.bot_status as string)}
            </p>
            <p>
                {t("Server")} : {appContext.server_address as string}
            </p>
            <p>Pack : {appContext.resource_pack_str as string}</p>
        </div>
    );
}
