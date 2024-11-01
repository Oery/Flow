import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/ui/dialog';

import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/api/shell';
import { invoke } from '@tauri-apps/api/tauri';

const oauthUrl = new URL('https://id.twitch.tv/oauth2/authorize');
const params = new URLSearchParams(oauthUrl.search);
const scopes = [
    'channel:manage:predictions',
    'moderator:manage:announcements',
    'user:read:chat',
    'user:write:chat',
];

params.set('client_id', 'cig4pc07b7bxo207x8158v58r1i5pf');
params.set('response_type', 'code');
params.set('redirect_uri', 'http://localhost:8457');
params.set('scope', scopes.join(' '));
const oauthUrlString = `${oauthUrl.origin + oauthUrl.pathname}?${params}`;

export default function LoginDialogButton() {
    const [isOpen, setIsOpen] = useState(false);

    useEffect(() => {
        const unlisten = listen<boolean>('show-login-modal', (event) => {
            setIsOpen(event.payload);
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    return (
        <Dialog open={isOpen}>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Welcome to Flow</DialogTitle>
                    <DialogDescription>
                        Thanks for downloading Flow, if you have any questions or suggestions,
                        please contact me on Twitter. If you need help to set up Flow, you can find
                        help on the Discord.
                    </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                    <button
                        type='button'
                        className='text-sm p-3 px-5 rounded-full text-flow-primary'
                        onClick={() => open(oauthUrlString)}
                    >
                        Login with Twitch
                    </button>
                    <button
                        type='button'
                        className='text-sm p-3 px-5 rounded-full text-flow-primary'
                        onClick={() => invoke('close_login_modal')}
                    >
                        Offline Mode
                    </button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    );
}
