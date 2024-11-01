import HomeModule from '@/components/home-module';
import styles from '@/styles/Home.module.css';
import DebugMenu from '@/components/debug-menu';

import { useTranslation } from 'react-i18next';

export default function HomePage() {
    const { t } = useTranslation();
    const isDebug = false;

    return (
        <>
            <header>
                <h1 className='text-flow-primary text-[34px]'>{t('Home')}</h1>
            </header>

            <main className={styles.home}>
                <HomeModule title={t('Queuing Scene')} setting='scenes_enable' link='/scene' />
                <HomeModule title='Resource Packs' setting='pack_enable' link='/dynpack' />
                <HomeModule title={t('Server Address')} setting='server_enable' link='/dynip' />
                <HomeModule title={t('Music')} setting='music_enable' link='/music' />

                {isDebug && <DebugMenu />}
            </main>
        </>
    );
}
