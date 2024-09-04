import type { Emote } from "./Emote";

interface Streamer {
    color: string;
    id: string;
    display_name: string;
    avatar_url: string;
    emotes: Emote[];
}

export type { Streamer };
