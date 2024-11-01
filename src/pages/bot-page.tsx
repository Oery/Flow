import BackArrow from '@/components/ui/back-arrow';
import styles from '@/styles/Settings.module.css';
import BotInput from '@/components/ui/bot-input';
import Module from '@/components/module';
import { useSettings } from '@/hooks/settings';
import textStyles from '@/styles/TextInput.module.css';

import { useTranslation } from 'react-i18next';
import { open } from '@tauri-apps/api/shell';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';

export default function BotPage() {
    const { t } = useTranslation();
    const { settings } = useSettings();
    const [wizebotApiKey, setWizebotApiKey] = useState<string>('');

    const nightbotUrl =
        'https://nightbot.tv/oauth2/authorize?client_id=decbae04d2836ff99a244c5767d7a851&response_type=code&redirect_uri=http:%2F%2Flocalhost:8458&scope=channel%20commands';

    return (
        <>
            <header>
                <BackArrow destination='/' />
                <h1 className='text-flow-primary text-[34px]'>{t('Twitch Bot')}</h1>
            </header>

            <div className={styles.settings}>
                <p className='text-flow-primary'>
                    {t('Bot Developers limits how Flow can interact with your Bot.')}
                    <br />
                    {t(
                        'For the best experience, I recommend using Flow with a custom Bot or your own account.',
                    )}
                </p>
                <ul>
                    <li>
                        <strong>Your account</strong> have access to all your emotes for free.
                    </li>
                    <li>
                        <strong>Custom Bots</strong> are just another Twitch account. Flow handle
                        everything.
                    </li>
                    <li>
                        <strong>Nightbot</strong> will work with most features, but announcements
                        will have to use your account.
                    </li>
                    <li>
                        <strong>WizeBot</strong> can only update commands but you will have to
                        create them manually.
                    </li>
                </ul>
                <Module title={t('Twitch Bot')} column={true}>
                    <BotInput />
                </Module>

                {settings.twitch_bot === 'custom' && (
                    <Module title={t('Custom Bot')}>
                        <button type='button' onClick={() => invoke('start_custom_bot_auth')}>
                            Connect to Custom Bot
                        </button>
                    </Module>
                )}

                {settings.twitch_bot === 'nightbot' && (
                    <Module title={t('Nightbot')}>
                        <button
                            type='button'
                            onClick={() => {
                                open(nightbotUrl);
                                invoke('start_nightbot_server')
                                    .then(() => {
                                        console.log('Nightbot server started');
                                    })
                                    .catch((e) => {
                                        console.error('Error while starting nightbot server : ', e);
                                    });
                            }}
                        >
                            {t('Connect to Nightbot')}
                        </button>
                    </Module>
                )}

                {settings.twitch_bot === 'wizebot' && (
                    <Module title={t('WizeBot API Key')} column={true}>
                        <input
                            className={textStyles.textinput}
                            type='password'
                            id='wizebot_apikey'
                            placeholder='Enter your Wizebot API Key'
                            value={wizebotApiKey}
                            onChange={(e) => {
                                setWizebotApiKey(e.target.value);
                                invoke('set_bot_token', {
                                    bot: 'wizebot',
                                    token: e.target.value,
                                })
                                    .then(() => {
                                        console.log('Wizebot token set');
                                    })
                                    .catch((e) => {
                                        console.error('Error while setting Wizebot token : ', e);
                                    });
                            }}
                        />
                    </Module>
                )}
            </div>
        </>
    );
}
