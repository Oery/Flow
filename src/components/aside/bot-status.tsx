import { useSettings } from '@/hooks/settings';
import { useAppContext } from '@/hooks/app-context';

import { useTranslation } from 'react-i18next';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api';

const botDisplayNames: Record<string, string> = {
    self: 'Self',
    custom: 'Custom',
    nightbot: 'Nightbot',
    wizebot: 'WizeBot',
};

export default function BotStatus() {
    const { settings } = useSettings();
    const appContext = useAppContext();
    const { t } = useTranslation();

    useEffect(() => {
        invoke('set_current_bot', { bot: settings.twitch_bot });
    }, [settings.twitch_bot]);

    return (
        <div>
            <h3 className='text-flow-primary text-2xl'>{botDisplayNames[settings.twitch_bot]}</h3>
            {/* Ajouter un bouton rouge ! pour ouvrir le modal de Nightbot en cas de token invalide */}
            <p className='text-flow-primary'>
                {t('Status')} : {t(appContext.bot_status)}
            </p>
            <p className='text-flow-primary'>
                {t('Server')} : {appContext.server_address}
            </p>
            <p className='text-flow-primary'>Pack : {appContext.resource_pack_str}</p>
        </div>
    );
}
