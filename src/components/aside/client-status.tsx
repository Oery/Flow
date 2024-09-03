import { t } from "i18next";
import { useAppContext } from "../AppContext";

export default function ClientStatus() {
    const { client, ingame_status, server_raw, resource_packs_raw } = useAppContext();

    if (client === undefined || client === "?") {
        return null;
    }

    return (
        <div>
            <h3>{client as string}</h3>
            <p>{`${t("Server")} : ${server_raw ?? "?"}`}</p>

            {resource_packs_raw && <p>Pack : {resource_packs_raw[0] as string}</p>}

            {ingame_status !== "Unknown" && (
                <p>{`${t("Status")} : ${t(ingame_status as string)}`}</p>
            )}
        </div>
    );
}
