import Titlebar from '@/components/title-bar';
import LoadingScreen from '@/components/loading-screen';
import AsideInfo from '@/components/aside/aside-infos';
import { SettingsProvider } from '@/hooks/settings';
import { AppProvider } from '@/hooks/app-context';
import styles from '@/styles/Root.module.css';
import '@/styles/globals.css';

import { useEffect, useState } from 'react';
import { type Event, listen } from '@tauri-apps/api/event';
import { Outlet } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/tauri';
import Nav from '@/components/nav';

export default function Root() {
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const [showLoadingScreen, setShowLoadingScreen] = useState<boolean>(true);

    useEffect(() => {
        const unlisten = listen('loading-end', (event: Event<boolean>) => {
            if (event.payload) {
                setShowLoadingScreen(false);
                invoke('start_event_loop');
                setTimeout(() => {
                    setIsLoading(false);
                }, 500);
            }
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    return (
        <main className='bg-flow-secondary w-screen'>
            <title>Flow</title>
            <SettingsProvider>
                <AppProvider>
                    {isLoading ? (
                        <LoadingScreen isLoading={showLoadingScreen} />
                    ) : (
                        <>
                            <Titlebar />

                            <div className={styles.maincontainer}>
                                <Nav />
                                <main className=' bg-flow-secondary'>
                                    <Outlet />
                                </main>
                                <AsideInfo />
                            </div>
                        </>
                    )}
                </AppProvider>
            </SettingsProvider>
        </main>
    );
}
