import { useSettings } from "../SettingsContext";
import styles from "../../styles/SelectInput.module.css";

interface Props {
    setting: string;
    choices: string[];
    placeholder: string;
}

export default function SelectInput({ setting, choices, placeholder }: Props) {
    const { settings, updateSetting } = useSettings();

    const handleChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
        updateSetting(setting, event.target.value);
    };

    return (
        <>
            {choices.length > 0 && (
                <select
                    onChange={handleChange}
                    value={settings[setting] as string}
                    className={styles.selectinput}
                >
                    {choices.map((choice) => (
                        <option key={choice} value={choice}>
                            {choice}
                        </option>
                    ))}
                </select>
            )}

            {choices.length === 0 && (
                <select disabled className={styles.selectinput}>
                    <option value="No scene">{placeholder}</option>
                </select>
            )}
        </>
    );
}
