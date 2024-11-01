const botFeatures: Record<string, string[]> = {
    self: ['custom_commands', 'edit_commands'],
    custom: ['custom_commands', 'edit_commands'],
    nightbot: ['custom_commands', 'edit_commands'],
    wizebot: ['edit_commands'],
};

export function getBotFeatures(bot: string): string[] {
    return botFeatures[bot];
}

export function hasBotFeature(bot: string, feature: string): boolean {
    return getBotFeatures(bot).includes(feature);
}
