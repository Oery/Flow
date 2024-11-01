import Spinner from '@/components/ui/spinner';
import Titlebar from '@/components/title-bar';
import styles from '@/styles/LoadingScreen.module.css';

import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api';
import LoginDialogButton from './ui/login-dialog-button';

interface Props {
    isLoading: boolean;
}

export default function LoadingScreen({ isLoading }: Props) {
    useEffect(() => {
        setTimeout(() => {
            invoke('load_app');
        }, 100);
    }, []);

    return (
        <div
            className={`flex flex-col items-center justify-center h-screen gap-11 bg-flow-secondary ${isLoading ? 'opacity-100' : styles.end}`}
        >
            <Titlebar />
            <h1 className='text-flow-primary text-7xl'>Flow</h1>
            <Spinner />
            <LoginDialogButton />
        </div>
    );
}
