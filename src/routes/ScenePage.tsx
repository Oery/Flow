import BackArrow from "../components/BackArrow";
import Module from "../components/Module";
import TextInput from "../components/TextInput";
import ToggleButton from "../components/ToggleButton";

import styles from "../styles/Settings.module.css";

export default function ScenePage() {
    return (
        <>
            <header>
                <BackArrow destination="/" />
                <h1>Queuing Scene</h1>
                <ToggleButton setting="scenes_enable" />
            </header>

            <div className={styles.settings}>
                <Module title="Queuing Scene Name" column={true}>
                    {/* Get OBS Scenes List */}
                    <TextInput
                        setting="scenes_name"
                        placeholder="Queuing Scene"
                    />
                </Module>

                <Module title="Toggle Scene after Game End">
                    <ToggleButton setting="scenes_toggle_after_game_end" />
                </Module>

                <Module title="Manual OBS Configuration">
                    <ToggleButton setting="scenes_manual_obs_config" />
                </Module>

                <Module
                    title={["Websocket Host", "Websocket Password"]}
                    column={true}
                >
                    <TextInput setting="websocket-host" placeholder="1.1.1.1" />
                    <TextInput
                        setting="websocket-password"
                        password={true}
                        placeholder="password1234"
                    />
                </Module>
            </div>
        </>
    );
}
