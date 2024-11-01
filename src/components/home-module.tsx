import { useSettings } from '@/hooks/settings';
import type { Settings } from '@/types/Settings';

import { Link } from 'react-router-dom';

interface Props {
    title: string;
    setting: keyof Settings;
    link: string;
}

export default function HomeModule({ title, link, setting }: Readonly<Props>) {
    const { settings, updateSetting } = useSettings();

    const handleClick = () => {
        updateSetting(setting, !settings[setting]);
    };

    return (
        <div className='bg-flow-secondary h-[100px] flex border border-[#3a3a3a] rounded-[20px] justify-between cursor-pointer'>
            <Link to={link} className='py-8 px-6'>
                <h3 className='text-flow-primary text-2xl'>{title}</h3>
            </Link>

            {/* biome-ignore lint/a11y/useKeyWithClickEvents: <explanation> */}
            <div
                onClick={handleClick}
                className={`aspect-square h-[98px] select-none text-2xl flex justify-center items-center border-l border-[#3a3a3a] rounded-tr-[20px] rounded-br-[20px] text-flow-primary transition duration-300   ${settings[setting] ? 'bg-flow-primary text-flow-secondary' : ''}`}
            >
                {settings[setting] ? 'On' : 'Off'}
            </div>
        </div>
    );
}
