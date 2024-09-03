import { invoke } from "@tauri-apps/api/tauri";
import { AliasGroup } from "../types/Alias";

export function updateTauriSetting(key: string, value: boolean | string) {
    invoke("update_setting", { key, value })
        .then(() => console.log("Setting updated successfully"))
        .catch((error) => console.error("Failed to update setting:", error));
}

export function updateTauriAlias(group: AliasGroup, name: string, alias: string) {
    invoke("update_alias", { group, name, alias })
        .then(() => console.log("Alias updated successfully", name, alias))
        .catch((error) => console.error("Failed to update alias:", error));
}

export function removeTauriAlias(group: AliasGroup, name: string) {
    invoke("delete_alias", { group, name })
        .then(() => console.log("Alias removed successfully"))
        .catch((error) => console.error("Failed to remove alias:", error));
}
