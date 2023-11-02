import styles from "../styles/TextInput.module.css";
import { useSettings } from "./SettingsContext";

interface Props {
    setting: string;
    placeholder: string;
    password?: boolean;
}

interface StringChangeEvent {
    target: {
        value: string;
    };
}

export default function TextInput({ setting, placeholder, password }: Props) {
    const { settings, updateSetting } = useSettings();

    const handleChange = ({ target }: StringChangeEvent) => {
        updateSetting(setting, target.value);
    };

    return (
        <input
            type={password ? "password" : "text"}
            id={setting}
            placeholder={placeholder}
            className={styles.textinput}
            autoComplete="off"
            spellCheck="false"
            value={settings[setting] as string}
            onChange={handleChange}
        />
    );
}
