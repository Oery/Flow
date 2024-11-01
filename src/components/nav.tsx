import NavElement from '@/components/ui/nav-element';

import { useTranslation } from 'react-i18next';
import { appWindow } from '@tauri-apps/api/window';

export default function Nav() {
    const { t } = useTranslation();

    return (
        <div className='h-screen w-[200px] px-[52px] border-r border-flow-border'>
            <header>
                <h1 className='text-flow-primary text-[34px]'>Flow</h1>
            </header>
            <nav>
                <ul>
                    <div>
                        <NavElement title={t('Home')} link='' />
                        <NavElement title={t('Scene')} link='scene' />
                        <NavElement title={t('Packs')} link='dynpack' />
                        <NavElement title={t('Server')} link='dynip' />
                        <NavElement title={t('Music')} link='music' />
                        <NavElement title={t('Bot')} link='bot' />
                    </div>

                    <footer>
                        <NavElement title={t('Settings')} link='settings' />
                        <NavElement
                            title={t('Exit')}
                            link='log-out'
                            handleClick={() => {
                                appWindow.close();
                            }}
                        />
                    </footer>
                </ul>
            </nav>
        </div>
    );
}
