import BackArrow from "../components/BackArrow";
import Module from "../components/Module";
import TextInput from "../components/TextInput";
import ToggleButton from "../components/ToggleButton";

import styles from "../styles/Settings.module.css";

export default function MusicPage() {
    return (
        <>
            <header>
                <BackArrow destination="/" />
                <h1>Music</h1>
                <ToggleButton setting="music_enable" />
            </header>

            <div className={styles.settings}>
                <Module title="Command Text" column={true}>
                    <TextInput
                        setting="music_command_text"
                        placeholder="🎵 : {music} - {artist}"
                    />
                </Module>

                <Module title="Announce on update">
                    <ToggleButton setting="music_announcements_enable" />
                </Module>

                <Module title="Announce Text" column={true}>
                    <TextInput
                        setting="music_announce_text"
                        placeholder="🎵 : {music} - {artist}"
                    />
                </Module>

                {/* DELAY OPTION FOR ANNOUNCE */}
            </div>
        </>
    );
}
