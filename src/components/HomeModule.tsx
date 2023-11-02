import { Link } from "react-router-dom";
import style from "../styles/HomeModule.module.css";
import { useSettings } from "./SettingsContext";

interface Props {
    title: string;
    module: string;
    link: string;
}

export default function HomeModule({ title, link, module }: Props) {
    const { settings, updateSetting } = useSettings();

    const handleClick = () => {
        updateSetting(`${module}_enable`, !settings[`${module}_enable`]);
    };

    return (
        <div className={style.homemodule}>
            <Link to={link}>
                <h3>{title}</h3>
            </Link>

            <div
                className={settings[`${module}_enable`] ? style.toggled : ""}
                onClick={handleClick}
            >
                {settings[`${module}_enable`] ? "On" : "Off"}
            </div>
        </div>
    );
}
