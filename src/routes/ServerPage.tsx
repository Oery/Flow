import BackArrow from "../components/BackArrow";
import Module from "../components/Module";
import TextInput from "../components/TextInput";
import ToggleButton from "../components/ToggleButton";

import styles from "../styles/Settings.module.css";

export default function ServerPage() {
    return (
        <>
            <header>
                <BackArrow destination="/" />
                <h1>Server Address</h1>
                <ToggleButton setting="server_enable" />
            </header>

            <div className={styles.settings}>
                <Module title="Command Text" column={true}>
                    <TextInput
                        setting="server_command_text"
                        placeholder="The server I'm playing on : {server}"
                    />
                </Module>

                <Module title="Announce">
                    <ToggleButton setting="server_announcements_enable" />
                </Module>

                <Module title="Announce Text" column={true}>
                    <TextInput
                        setting="server_announcements_text"
                        placeholder="I just joined : {server}"
                    />
                </Module>
            </div>
        </>
    );
}
