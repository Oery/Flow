import BackArrow from "../components/BackArrow";
import Module from "../components/Module";
import TextInput from "../components/TextInput";
import ToggleButton from "../components/ToggleButton";

import styles from "../styles/Settings.module.css";

export default function PackPage() {
    return (
        <>
            <header>
                <BackArrow destination="/" />
                <h1>Resource Pack</h1>
                <ToggleButton setting="pack_enable" />
            </header>

            <div className={styles.settings}>
                <Module title="Command Text" column={true}>
                    <TextInput
                        setting="pack_command_text"
                        placeholder="Resource Pack : {pack}"
                    />
                </Module>

                <Module title="Announce">
                    <ToggleButton setting="pack_announcements_enable" />
                </Module>

                <Module title="Announce Text" column={true}>
                    <TextInput
                        setting="pack_announcements_text"
                        placeholder="Resource Pack : {pack}"
                    />
                </Module>
            </div>
        </>
    );
}
