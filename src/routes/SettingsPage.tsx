import BackArrow from "../components/BackArrow";
import BotInput from "../components/BotInput";
import MCClientInput from "../components/MCClientInput";
import Module from "../components/Module";
import ToggleButton from "../components/ToggleButton";

import styles from "../styles/Settings.module.css";

function SettingsPage() {
    return (
        <>
            <header>
                <BackArrow destination="/" />
                <h1>Settings</h1>
            </header>

            <div className={styles.settings}>
                <Module title={"Streaming Only"}>
                    <ToggleButton setting="streaming_only" />
                </Module>
                <Module title={"Streaming Minecraft Only"}>
                    <ToggleButton setting="streaming_mc_only" />
                </Module>
                <Module title={"Minecraft Client"}>
                    <MCClientInput />
                </Module>
                <Module title={"Twitch Bot"}>
                    <BotInput></BotInput>
                </Module>
                <Module title={"Disable Hardware Acceleration"}>
                    <ToggleButton setting="disable-hardware-acceleration" />
                </Module>
            </div>
        </>
    );
}

export default SettingsPage;
