import { useSettings } from '@/hooks/settings';
import styles from '@/styles/SelectInput.module.css';
import type { Settings } from '@/types/Settings';

interface Props {
    setting: keyof Settings;
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
                    value={settings[setting]}
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
                    <option value='No scene'>{placeholder}</option>
                </select>
            )}
        </>
    );
}
