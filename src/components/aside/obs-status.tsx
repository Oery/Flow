import spinnerStyles from '@/styles/SpinnerSec.module.css';
import { useAppContext } from '@/hooks/app-context';

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { invoke } from '@tauri-apps/api/tauri';

export default function ObsStatus() {
    const appContext = useAppContext();
    const [loading, setLoading] = useState<boolean>(false);
    const { t } = useTranslation();

    const connectToObs = () => {
        setLoading(true);
        invoke('connect_to_obs')
            .then(() => {
                setLoading(false);
                appContext.obs_status = 'Online';
            })
            .catch(() => setLoading(false));
    };

    return (
        <div>
            <h3 className='text-flow-primary text-2xl'>OBS</h3>
            <p className='text-flow-primary'>
                {t('Status')} : {t(appContext.obs_status)}
                {appContext.obs_status === 'Offline' && (
                    <span
                        onKeyDown={!loading ? connectToObs : undefined}
                        onClick={!loading ? connectToObs : undefined}
                        className={loading ? spinnerStyles.spinning : spinnerStyles.notspinning}
                    >
                        {' '}
                        ↺
                    </span>
                )}
            </p>
        </div>
    );
}
