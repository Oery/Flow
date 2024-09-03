import BackArrow from "../components/ui/back-arrow";
import ToggleButton from "../components/ui/toggle-button";

import styles from "../styles/Settings.module.css";

export default function StatsPage() {
    return (
        <>
            <header>
                <BackArrow destination="/" />
                <h1>Stats</h1>
                <ToggleButton setting="stats_enable" />
            </header>

            <div className={styles.settings}>
                {/* <Module title={"Labels Folder"} column={true}>
                    <TextInput
                        setting="stats_labels_folder"
                        placeholder="Path to labels folder"
                    />
                </Module>
                <Module title={"Username"} column={true}>
                    <TextInput
                        setting="stats_username"
                        placeholder="Username"
                    />
                </Module> */}
            </div>
        </>
    );
}
