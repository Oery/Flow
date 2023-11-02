import HomeModule from "../components/HomeModule";
import styles from "../styles/Home.module.css";

export default function HomePage() {
    return (
        <>
            <header>
                <h1>Home</h1>
            </header>

            <main className={styles.home}>
                <HomeModule
                    title="Queuing Scene"
                    module="scenes"
                    link="/scene"
                />
                <HomeModule
                    title="Resource Packs"
                    module="pack"
                    link="/dynpack"
                />
                <HomeModule
                    title="Server Address"
                    module="server"
                    link="/dynip"
                />
                <HomeModule title="Music" module="music" link="/music" />
            </main>
        </>
    );
}
