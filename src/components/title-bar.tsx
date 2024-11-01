import styles from '@/styles/Titlebar.module.css';

import { appWindow } from '@tauri-apps/api/window';

export default function Titlebar() {
    const handleMaximizeClick = async () => {
        const isMaximized = await appWindow.isMaximized();
        if (!isMaximized) appWindow.maximize();
        else appWindow.unmaximize();
    };

    return (
        <div className={styles.titlebar}>
            <div data-tauri-drag-region>
                <div className={`${styles.buttons}`}>
                    {/* biome-ignore lint/a11y/useKeyWithClickEvents: <explanation> */}
                    <div onClick={() => appWindow.minimize()}>
                        <svg x='0px' y='0px' viewBox='0 0 10.2 1'>
                            <title id='minimizeTitle'>Minimize</title>
                            <rect x='0' y='50%' width='10.2' height='1' />
                        </svg>
                    </div>
                    {/* biome-ignore lint/a11y/useKeyWithClickEvents: <explanation> */}
                    <div onClick={handleMaximizeClick}>
                        <svg viewBox='0 0 10 10'>
                            <title id='maximizeTitle'>Maximize</title>
                            <path d='M0,0v10h10V0H0z M9,9H1V1h8V9z' />
                        </svg>
                    </div>
                    {/* biome-ignore lint/a11y/useKeyWithClickEvents: <explanation> */}
                    <div onClick={() => appWindow.hide()}>
                        <svg viewBox='0 0 10 10'>
                            <title id='hideTitle'>Hide</title>
                            <polygon points='10.2,0.7 9.5,0 5.1,4.4 0.7,0 0,0.7 4.4,5.1 0,9.5 0.7,10.2 5.1,5.8 9.5,10.2 10.2,9.5 5.8,5.1' />
                        </svg>
                    </div>
                </div>
            </div>
        </div>
    );
}
