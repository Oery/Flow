import BackArrow from '@/components/ui/back-arrow';
import Module from '@/components/module';
import { useSettings } from '@/hooks/settings';
import TextInput from '@/components/ui/text-input';
import ToggleButton from '@/components/ui/toggle-button';
import type { Scene, Scenes } from '@/types/Scene';
import SelectInput from '@/components/ui/select-input';
import styles from '@/styles/Settings.module.css';

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api';
import { useTranslation } from 'react-i18next';

export default function ScenePage() {
    const { settings } = useSettings();
    const [scenes, setScenes] = useState<Scene[]>([]);
    const { t } = useTranslation();

    useEffect(() => {
        invoke<Scenes>('get_scenes_list')
            .then((data) => setScenes(data.scenes))
            .catch((err) => console.error('Error getting scenes list:', err));
    }, []);

    return (
        <>
            <header>
                <BackArrow destination='/' />
                <h1 className='text-flow-primary text-[34px]'>{t('Queuing Scene')}</h1>
                <ToggleButton setting='scenes_enable' />
            </header>

            <div className={styles.settings}>
                <Module title={t('Queuing Scene Name')} column={true}>
                    <SelectInput
                        setting='scenes_name'
                        choices={scenes.map((scene) => scene.sceneName)}
                        placeholder={t('Please connect Flow to OBS to access your scenes')}
                    />
                </Module>

                <Module title={t('Toggle Scene after Game End')}>
                    <ToggleButton setting='scenes_toggle_after_game_end' />
                </Module>

                <Module title={t('Keep Screen hidden in Lobby')}>
                    <ToggleButton setting='scenes_hide_in_lobby' />
                </Module>

                <Module title={t('Auto OBS Configuration')}>
                    <ToggleButton setting='scenes_auto_obs_config' />
                </Module>

                {settings.scenes_auto_obs_config && (
                    <Module title={t('OBS Config Path')} column={true}>
                        <TextInput
                            setting='scenes_obs_config_path'
                            placeholder='%APPDATA%\obs-studio'
                        />
                    </Module>
                )}

                {!settings.scenes_auto_obs_config && (
                    <Module title={[t('Websocket Host'), t('Websocket Password')]} column={true}>
                        <TextInput setting='scenes_obs_ws_port' placeholder='4455' />
                        <TextInput
                            setting='scenes_obs_ws_password'
                            password={true}
                            placeholder='password1234'
                        />
                    </Module>
                )}
            </div>
        </>
    );
}
