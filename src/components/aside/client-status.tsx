import { t } from 'i18next';
import { useAppContext } from '../../hooks/app-context';

export default function ClientStatus() {
    const { client, ingame_status, server_raw, resource_packs_raw } = useAppContext();

    if (client === undefined || client === '?') {
        return null;
    }

    return (
        <div>
            <h3 className='text-flow-primary text-2xl'>{client}</h3>
            <p>{`${t('Server')} : ${server_raw ?? '?'}`}</p>

            {resource_packs_raw && <p>Pack : {resource_packs_raw[0]}</p>}

            {ingame_status !== 'Unknown' && <p>{`${t('Status')} : ${t(ingame_status)}`}</p>}
        </div>
    );
}
