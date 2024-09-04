import type { Aliases } from "./Alias";
import type { Streamer } from "./Streamer";

interface AliasStore {
    packs: Aliases;
    servers: Aliases;
}

interface App {
    [key: string]: boolean | string | Streamer | string[] | AliasStore;
    streamer: Streamer;
    resource_packs_raw: string[];
    aliases: AliasStore;
}

export type { App };
