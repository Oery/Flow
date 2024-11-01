import type { Aliases } from '@/types/Aliases';

import { invoke } from '@tauri-apps/api/tauri';

export function updateTauriSetting(key: string, value: unknown) {
    invoke('update_setting', { key, value })
        .then(() => console.log('Setting updated successfully'))
        .catch((error) => console.error('Failed to update setting:', error));
}

export function updateTauriAlias(group: keyof Aliases, name: string, alias: string) {
    invoke('update_alias', { group, name, alias })
        .then(() => console.log('Alias updated successfully', name, alias))
        .catch((error) => console.error('Failed to update alias:', error));
}

export function removeTauriAlias(group: keyof Aliases, name: string) {
    invoke('delete_alias', { group, name })
        .then(() => console.log('Alias removed successfully'))
        .catch((error) => console.error('Failed to remove alias:', error));
}
