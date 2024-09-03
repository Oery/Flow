type AliasGroup = "packs" | "servers";

interface Alias {
    alias: string;
    hidden: boolean;
}

interface Aliases {
    [key: string]: Alias;
}

export type { AliasGroup, Alias, Aliases };
