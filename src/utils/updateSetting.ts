import { invoke } from "@tauri-apps/api/tauri";

export function updateTauriSetting(key: string, value: boolean | string) {
    invoke("update_setting", { key, value })
        .then(() => {
            console.log("Setting updated successfully");
        })
        .catch((error) => {
            console.error("Failed to update setting:", error);
        });
}
