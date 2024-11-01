import BackArrow from '@/components/ui/back-arrow';
import Module from '@/components/module';
import ToggleButton from '@/components/ui/toggle-button';
import { useSettings } from '@/hooks/settings';
import styles from '@/styles/Settings.module.css';
import selectstyles from '@/styles/SelectInput.module.css';

import { enable, disable } from 'tauri-plugin-autostart-api';
import { useTranslation } from 'react-i18next';

function SettingsPage() {
    const { t, i18n } = useTranslation();
    const { updateSetting } = useSettings();

    const handleLangChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
        updateSetting('language', event.target.value);
        i18n.changeLanguage(event.target.value);
    };

    return (
        <>
            <header>
                <BackArrow destination='/' />
                <h1 className='text-flow-primary text-[34px]'>{t('Settings')}</h1>
            </header>

            <div className={styles.settings}>
                <Module title={t('Language')}>
                    <select
                        className={selectstyles.selectinput}
                        style={{
                            width: 'fit-content',
                        }}
                        name='language'
                        id='language'
                        onChange={handleLangChange}
                        value={i18n.language}
                    >
                        <option value='en'>English</option>
                        <option value='fr'>Français</option>
                    </select>
                </Module>

                <Module title={t('Start with Windows')}>
                    <ToggleButton
                        setting='start_with_windows'
                        callback={(value) => {
                            const task = value ? disable() : enable();
                            task.then(() => console.log('Start with Windows changed to', value));
                        }}
                    />
                </Module>

                {/* TODO: Add those features */}
                {/* <Module title={t("Streaming Only")}>
                    <ToggleButton setting="streaming_only" />
                </Module>

                {settings.streaming_only && (
                    <Module title={t("Only in Minecraft category")}>
                        <ToggleButton setting="streaming_mc_only" />
                    </Module>
                )} */}

                {/* <Module title={t("Disable Hardware Acceleration")}>
                    <ToggleButton setting="disable-hardware-acceleration" />
                </Module> */}
            </div>
        </>
    );
}

export default SettingsPage;
