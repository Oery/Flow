import { useTranslation } from "react-i18next";
import HomeModule from "../components/HomeModule";
import styles from "../styles/Home.module.css";
import DebugMenu from "../components/debug-menu";

export default function HomePage() {
    const { t } = useTranslation();
    const isDebug = false;

    return (
        <>
            <header>
                <h1>{t("Home")}</h1>
            </header>

            <main className={styles.home}>
                <HomeModule title={t("Queuing Scene")} module="scenes" link="/scene" />
                <HomeModule title="Resource Packs" module="pack" link="/dynpack" />
                <HomeModule title={t("Server Address")} module="server" link="/dynip" />
                <HomeModule title={t("Music")} module="music" link="/music" />
                {/* <HomeModule title="Stats" module="stats" link="/stats" /> */}

                {isDebug && <DebugMenu />}
            </main>
        </>
    );
}
