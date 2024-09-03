// import { useRef } from "react";
// import BotModalWizeBot from "./BotModalWizeBot";
// import BotModalNightbot from "./BotModalNightbot";
// import BotModalCustom from "./BotModalCustom";
import { useSettings } from "../SettingsContext";
import { invoke } from "@tauri-apps/api/tauri";

import styles from "../../styles/SelectInput.module.css";

export default function BotInput() {
    // const nightbot = useRef<HTMLDialogElement | null>(null);
    // const wizebot = useRef<HTMLDialogElement | null>(null);
    // const custom = useRef<HTMLDialogElement | null>(null);
    const { settings, updateSetting } = useSettings();

    function handleChange(bot: string) {
        trySetBot(bot);
    }

    function trySetBot(bot: string) {
        invoke("set_current_bot", { bot: bot as string })
            .then(() => updateSetting("twitch_bot", bot))
            .catch((e) => console.error("Error setting bot:", e));
    }

    // function setBot(bot: string) {
    //     switch (bot) {
    //         case "Nightbot":
    //             updateSetting("twitch_bot", "nightbot");
    //             nightbot.current?.showModal();
    //             break;

    //         case "WizeBot":
    //             updateSetting("twitch_bot", "wizebot");
    //             wizebot.current?.showModal();
    //             break;

    //         case "Custom":
    //             custom.current?.showModal();
    //             break;
    //     }
    // }

    return (
        <>
            <select
                onChange={(e) => handleChange(e.target.value)}
                className={styles.selectinput}
                value={settings.twitch_bot as string}
                placeholder="Select a bot"
            >
                <option value="self">Self</option>
                <option value="custom">Custom</option>
                <option value="nightbot">Nightbot</option>
                <option value="wizebot">WizeBot</option>
            </select>

            {/* <BotModalWizeBot ref={wizebot} /> */}
            {/* <BotModalNightbot ref={nightbot} /> */}
            {/* <BotModalCustom ref={custom} /> */}
        </>
    );
}
