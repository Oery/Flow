import { useSettings } from "../SettingsContext";
import styles from "../../styles/Alias.module.css";
import textStyles from "../../styles/TextInput.module.css";
import { useState } from "react";
import { invoke } from "@tauri-apps/api";
import { AliasGroup } from "../../types/Alias";

type Timeout = ReturnType<typeof setTimeout>;

interface Props {
    group: AliasGroup;
    name: string;
}

export default function AliasInput({ group, name }: Props) {
    const { settings, updateAlias, removeAlias } = useSettings();
    const [debounceTimeout, setDebounceTimeout] = useState<Timeout | null>(null);

    console.log("Aliases : ", settings[group]);
    console.log("Settings : ", settings);

    const thisAlias = settings.aliases[group]
        ? settings.aliases[group][name]
        : {
              alias: "",
              hidden: false,
          };

    // const toggleHidden = () => {
    //     settings.updateSetting(`${name}_hidden`, !data.hidden);
    // };

    const updateCommand = () => {
        invoke("update_command", { group }).then((_) => console.log("Updated command"));
    };

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.value === "") {
            removeAlias(group, name);
            return;
        }

        updateAlias(group, name, e.target.value);

        if (debounceTimeout) clearTimeout(debounceTimeout);
        const newTimeout = setTimeout(() => {
            updateCommand();
        }, 1000);

        setDebounceTimeout(newTimeout);
    };

    const handleDelete = () => {
        removeAlias(group, name);
        invoke("update_command", { group }).then((_) => console.log("Updated command"));
    };

    return (
        <div key={name} className={styles.alias}>
            <p>{name}</p>
            <input
                className={`${textStyles.textinput} ${textStyles.small}`}
                type="text"
                placeholder={name}
                value={thisAlias ? thisAlias.alias : ""}
                onChange={handleChange}
            />
            <button type="button" onClick={handleDelete}>
                DELETE
            </button>
        </div>
    );
}
