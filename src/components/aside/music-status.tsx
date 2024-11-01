import { useTranslation } from 'react-i18next';
import { useAppContext } from '../../hooks/app-context';

export default function MusicStatus() {
    const appContext = useAppContext();
    const { t } = useTranslation();

    return (
        <div>
            <h3 className='text-flow-primary text-2xl'>{t('Music')}</h3>
            <p className='text-flow-primary'>
                {t('Title')} : {appContext.song_title}
            </p>
            <p className='text-flow-primary'>
                {t('Artist')} : {appContext.song_author}
            </p>
        </div>
    );
}
