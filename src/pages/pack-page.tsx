import { useTranslation } from "react-i18next";
import BackArrow from "../components/ui/back-arrow";
import Module from "../components/Module";
import TextInput from "../components/ui/text-input";
import ToggleButton from "../components/ui/toggle-button";
import { useSettings } from "../components/SettingsContext";

import styles from "../styles/Settings.module.css";
import { useAppContext } from "../components/AppContext";
import AliasInput from "../components/ui/alias-input";
import { invoke } from "@tauri-apps/api";

export default function PackPage() {
    const { t } = useTranslation();
    const { settings } = useSettings();
    const appContext = useAppContext();

    return (
        <>
            <header>
                <BackArrow destination="/" />
                <h1>{t("Resource Packs")}</h1>
                <ToggleButton setting="pack_enable" />
            </header>

            <div className={styles.settings}>
                <Module title={t("Command Text")} column={true}>
                    <TextInput
                        group="packs"
                        setting="pack_command_text"
                        placeholder="Resource Pack : {pack}"
                    />
                </Module>

                <Module title={t("Hide Overlay")}>
                    <ToggleButton
                        setting="pack_hide_overlay"
                        callback={() => {
                            invoke("hide_overlay", {
                                shouldHide: !settings.pack_hide_overlay,
                            }).then(() => {});
                        }}
                    />
                </Module>

                <Module title={t("Announce")}>
                    <ToggleButton setting="pack_announcements_enable" />
                </Module>

                {settings.pack_announcements_enable && (
                    <Module title={t("Announce Text")} column={true}>
                        <TextInput
                            group="packs"
                            setting="pack_announcements_text"
                            placeholder="Resource Pack : {pack}"
                        />
                    </Module>
                )}

                <Module title={t("Your Packs")} column={true}>
                    {/* Current Packs */}
                    {appContext.resource_packs_raw.map((pack) => (
                        <AliasInput key={pack} group="packs" name={pack} />
                    ))}

                    <p />

                    {/* Aliases set by user */}
                    {Object.keys(settings.aliases.packs)
                        .filter((alias) => !appContext.resource_packs_raw.includes(alias))
                        .map((alias) => (
                            <AliasInput key={alias} group="packs" name={alias} />
                        ))}
                </Module>
            </div>
        </>
    );
}
