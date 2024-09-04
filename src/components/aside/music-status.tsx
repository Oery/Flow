import { useTranslation } from "react-i18next";
import { useAppContext } from "../AppContext";

export default function MusicStatus() {
    const appContext = useAppContext();
    const { t } = useTranslation();

    return (
        <div>
            <h3>{t("Music")}</h3>
            <p>
                {t("Title")} : {appContext.song_title as string}
            </p>
            <p>
                {t("Artist")} : {appContext.song_author as string}
            </p>
        </div>
    );
}
