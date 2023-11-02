import { useRef } from "react";
import BotModalWizeBot from "./BotModalWizeBot";
import BotModalNightbot from "./BotModalNightbot";
import BotModalCustom from "./BotModalCustom";

export default function BotInput() {
    const nightbot = useRef<HTMLDialogElement | null>(null);
    const wizebot = useRef<HTMLDialogElement | null>(null);
    const custom = useRef<HTMLDialogElement | null>(null);

    function handleChange(bot: string) {
        switch (bot) {
            case "Nightbot":
                nightbot.current?.showModal();
                break;

            case "WizeBot":
                wizebot.current?.showModal();
                break;

            case "Custom":
                custom.current?.showModal();
                break;
        }
    }

    return (
        <>
            <button onClick={() => handleChange("Nightbot")}>Click here</button>
            <select onChange={(e) => handleChange(e.target.value)}>
                <option value="Nightbot">Nightbot</option>
                <option value="WizeBot">WizeBot</option>
                <option value="Custom">Custom</option>
            </select>

            <BotModalWizeBot ref={wizebot} />
            <BotModalNightbot ref={nightbot} />
            <BotModalCustom ref={custom} />
        </>
    );
}
