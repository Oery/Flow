import { invoke } from '@tauri-apps/api/tauri';

export default function DebugMenu() {
    return (
        <>
            <button onClick={() => invoke('start_event_loop')} type='button'>
                Start Event Loop
            </button>

            <button onClick={() => invoke('disconnect_from_obs')} type='button'>
                Disconnect from OBS
            </button>

            <button onClick={() => invoke('start_custom_bot_auth')} type='button'>
                Open FlowBot Window
            </button>

            <button onClick={() => invoke('close_custom_bot_auth')} type='button'>
                Close FlowBot Window
            </button>

            <button onClick={() => invoke('log_out')} type='button'>
                Log out
            </button>
        </>
    );
}
