import { useAppContext } from "./AppContext";
import styles from "../styles/AsideInfo.module.css";

export default function AsideInfo() {
    const appContext = useAppContext();

    return (
        <aside className={styles.aside}>
            <header>
                <h1>{appContext.streamer.display_name}</h1>
            </header>

            <div>
                <div>
                    <h3>Your Client</h3>
                    <p>{`Client : ${appContext.client as string}`}</p>
                    <p>{`Server : ${appContext.client as string}`}</p>
                    <p>{`Pack : ${appContext.client as string}`}</p>
                    <p>{`Status : ${appContext.client as string}`}</p>
                </div>

                <div>
                    <h3>Your Bot</h3>
                    {/* Ajouter un bouton rouge ! pour ouvrir le modal de Nightbot en cas de token invalide */}
                    <p>Selected : Nightbot</p>
                    <p>Status : Online</p>
                </div>
            </div>
        </aside>
    );
}
