import styles from '@/styles/ToggleButton.module.css';
import { useSettings } from '@/hooks/settings';
import type { Settings } from '@/types/Settings';

interface Props {
    setting: keyof Settings;
    callback?: (value: boolean | string) => void;
}

function ToggleButton({ setting, callback }: Props) {
    const { settings, updateSetting } = useSettings();

    const handleClick = () => {
        updateSetting(setting, !settings[setting]);
        if (callback) callback(!settings[setting]);
    };

    return (
        <label className={styles.toggle}>
            <input
                type='checkbox'
                id={setting}
                checked={settings[setting] ? settings[setting] : false}
                onChange={handleClick}
            />
            <span className={styles.slider} />
        </label>
    );
}

export default ToggleButton;
