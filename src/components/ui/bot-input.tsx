import { useSettings } from '@/hooks/settings';
import styles from '@/styles/SelectInput.module.css';

import { invoke } from '@tauri-apps/api/tauri';

export default function BotInput() {
    const { settings, updateSetting } = useSettings();

    function handleChange(bot: string) {
        trySetBot(bot);
    }

    function trySetBot(bot: string) {
        invoke('set_current_bot', { bot })
            .then(() => updateSetting('twitch_bot', bot))
            .catch((e) => console.error('Error setting bot:', e));
    }

    return (
        <select
            onChange={(e) => handleChange(e.target.value)}
            className={styles.selectinput}
            value={settings.twitch_bot}
        >
            <option value='self'>Self</option>
            <option value='custom'>Custom</option>
            <option value='nightbot'>Nightbot</option>
            <option value='wizebot'>WizeBot</option>
        </select>
    );
}
