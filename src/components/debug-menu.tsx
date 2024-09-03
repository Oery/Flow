import { invoke } from "@tauri-apps/api/tauri";

export default function DebugMenu() {
    return (
        <>
            <button onClick={() => invoke("start_event_loop")} type="button">
                Start Event Loop
            </button>

            <button onClick={() => invoke("disconnect_from_obs")} type="button">
                Disconnect from OBS
            </button>
        </>
    );
}
