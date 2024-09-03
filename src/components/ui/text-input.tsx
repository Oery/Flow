import { invoke } from "@tauri-apps/api";
import styles from "../../styles/TextInput.module.css";
import { useSettings } from "../SettingsContext";
import { useState } from "react";

type Timeout = ReturnType<typeof setTimeout>;

interface Props {
    setting: string;
    group?: string;
    placeholder: string;
    password?: boolean;
}

export default function TextInput({ group, setting, placeholder, password }: Props) {
    const { settings, updateSetting } = useSettings();
    const [preview, setPreview] = useState<string>("");
    const [debounceTimeout, setDebounceTimeout] = useState<Timeout | null>(null);

    const updatePreview = (value: string) => {
        invoke<string>("preview_command", { cmdName: setting, cmdText: value }).then((res) =>
            setPreview(res)
        );
    };

    const updateCommand = () => {
        if (!group) return;
        invoke("update_command", { group, cmdName: setting }).then((_) =>
            console.log("Updated command")
        );
    };

    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        updateSetting(setting, event.target.value);
        updatePreview(event.target.value);

        if (debounceTimeout) clearTimeout(debounceTimeout);

        const newTimeout = setTimeout(() => {
            updateCommand();
        }, 1000);

        setDebounceTimeout(newTimeout);
    };

    return (
        <>
            <input
                type={password ? "password" : "text"}
                id={setting}
                placeholder={placeholder}
                className={styles.textinput}
                autoComplete="off"
                spellCheck="false"
                value={settings[setting] as string}
                onChange={handleChange}
                onBlur={() => setPreview("")}
                onFocus={(event) => updatePreview(event.target.value)}
            />
            {preview && <p>{preview}</p>}
        </>
    );
}
