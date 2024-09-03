import BackArrow from "../components/ui/back-arrow";
import Module from "../components/Module";
import TextInput from "../components/ui/text-input";
import ToggleButton from "../components/ui/toggle-button";
import { useTranslation } from "react-i18next";
import { useSettings } from "../components/SettingsContext";

import styles from "../styles/Settings.module.css";
import AliasInput from "../components/ui/alias-input";
import { useAppContext } from "../components/AppContext";
import { hasBotFeature } from "../utils/bot-features";

export default function ServerPage() {
    const { t } = useTranslation();
    const { settings } = useSettings();
    const appContext = useAppContext();

    return (
        <>
            <header>
                <BackArrow destination="/" />
                <h1>{t("Server Address")}</h1>
                <ToggleButton setting="server_enable" />
            </header>

            <div className={styles.settings}>
                {hasBotFeature(settings.twitch_bot as string, "custom_commands") && (
                    <Module title={t("Command")} column={true}>
                        <TextInput group="servers" setting="server_command" placeholder="!ip" />
                    </Module>
                )}

                <Module title={t("Command Text")} column={true}>
                    <TextInput
                        group="servers"
                        setting="server_command_text"
                        placeholder="IP : {server}"
                    />
                </Module>

                <Module title={t("Announce")}>
                    <ToggleButton setting="server_announcements_enable" />
                </Module>

                {settings.server_announcements_enable && (
                    <Module title={t("Announce Text")} column={true}>
                        <TextInput
                            group="servers"
                            setting="server_announcements_text"
                            placeholder="IP : {server}"
                        />
                    </Module>
                )}

                <Module title={t("Your Servers")} column={true}>
                    {appContext.server_raw && (
                        <AliasInput group="servers" name={appContext.server_raw as string} />
                    )}

                    <p />

                    {Object.keys(settings.aliases.servers)
                        .filter((alias) => alias !== appContext.server_raw)
                        .map((alias) => (
                            <AliasInput key={alias} group="servers" name={alias} />
                        ))}
                </Module>
            </div>
        </>
    );
}
