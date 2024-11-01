import BackArrow from "../components/ui/back-arrow";
import Module from "../components/Module";
import TextInput from "../components/ui/text-input";
import ToggleButton from "../components/ui/toggle-button";
import { useTranslation } from "react-i18next";

import styles from "../styles/Settings.module.css";
import { useSettings } from "../components/SettingsContext";
import { hasBotFeature } from "../utils/bot-features";

export default function MusicPage() {
    const { t } = useTranslation();
    const { settings } = useSettings();

    return (
        <>
            <header>
                <BackArrow destination="/" />
                <h1 className='text-flow-primary text-[34px]'>{t('Music')}</h1>
                <ToggleButton setting="music_enable" />
            </header>

            <div className={styles.settings}>
                {hasBotFeature(settings.twitch_bot as string, "custom_commands") && (
                    <Module title={t("Command")} column={true}>
                        <TextInput setting="music_command" placeholder="!music" />
                    </Module>
                )}

                <Module title={t("Command Text")} column={true}>
                    <TextInput setting="music_command_text" placeholder="🎵 : {title} - {artist}" />
                </Module>

                <Module title={t("Announce")}>
                    <ToggleButton setting="music_announcements_enable" />
                </Module>

                {settings.music_announcements_enable && (
                    <Module title={t("Announce Text")} column={true}>
                        <TextInput
                            setting="music_announce_text"
                            placeholder="🎵 : {title} - {artist}"
                        />
                    </Module>
                )}

                <Module title={t("Ignore Twitch")}>
                    <ToggleButton setting="music_ignore_twitch" />
                </Module>

                {/* DELAY OPTION FOR ANNOUNCE */}
            </div>
        </>
    );
}
