import styles from "../styles/ToggleButton.module.css";
import { useSettings } from "./SettingsContext";

interface Props {
    setting: string;
}

function ToggleButton({ setting }: Props) {
    const { settings, updateSetting } = useSettings();

    const handleClick = () => {
        updateSetting(setting, !settings[setting]);
    };

    return (
        <label className={styles.toggle}>
            <input
                type="checkbox"
                id={setting}
                checked={
                    (settings[setting] as boolean)
                        ? (settings[setting] as boolean)
                        : false
                }
                onChange={handleClick}
            />
            <span className={styles.slider} />
        </label>
    );
}

export default ToggleButton;
