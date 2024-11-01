import styles from '@/styles/TextInput.module.css';
import { useSettings } from '@/hooks/settings';
import { useAppContext } from '@/hooks/app-context';
import type { Emote } from '@/types/Emote';
import type { Settings } from '@/types/Settings';

import { useState } from 'react';
import { invoke } from '@tauri-apps/api';

type Timeout = ReturnType<typeof setTimeout>;

interface Props {
    setting: keyof Settings;
    group?: string;
    placeholder: string;
    password?: boolean;
}

export default function TextInput({ group, setting, placeholder, password }: Props) {
    const { settings, updateSetting } = useSettings();
    const appContext = useAppContext();
    const [preview, setPreview] = useState<(string | Emote)[]>([]);
    const [debounceTimeout, setDebounceTimeout] = useState<Timeout | null>(null);

    const updatePreview = (value: string) => {
        invoke<string>('preview_command', { cmdName: setting, cmdText: value }).then((res) => {
            const previewWithEmotes = res.split(' ').map((word) => {
                const emote = appContext.streamer.emotes.find((emote) => emote.name === word);
                if (emote) return emote;
                return `${word} `;
            });

            setPreview(previewWithEmotes);
        });
    };

    const updateCommand = () => {
        if (!group) return;
        invoke('update_command', { group, cmdName: setting }).then((_) =>
            console.log('Updated command'),
        );
    };

    const handlePreview = (input: string) => {
        if (setting.endsWith('_text')) {
            updatePreview(input);
        }
    };

    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        updateSetting(setting, event.target.value);
        handlePreview(event.target.value);

        if (debounceTimeout) clearTimeout(debounceTimeout);

        const newTimeout = setTimeout(() => {
            updateCommand();
        }, 1000);

        setDebounceTimeout(newTimeout);
    };

    return (
        <>
            <input
                type={password ? 'password' : 'text'}
                id={setting}
                placeholder={placeholder}
                className={`${styles.textinput} placeholder:text-[#545454]`}
                autoComplete='off'
                spellCheck='false'
                value={settings[setting]}
                onChange={handleChange}
                onBlur={() => setPreview([])}
                onFocus={(event) => handlePreview(event.target.value)}
            />
            {preview.length > 0 && (
                <p>
                    {preview.map((word, index) => {
                        if (typeof word === 'string') return word;
                        return (
                            <img
                                src={word.image_url}
                                alt={word.name}
                                key={`${word.name}-${index}`}
                                style={{ verticalAlign: 'middle' }}
                            />
                        );
                    })}
                </p>
            )}
        </>
    );
}
