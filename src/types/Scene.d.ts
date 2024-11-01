interface Scene {
    sceneIndex: number;
    sceneName: string;
}

interface Scenes {
    currentPreviewSceneName: string | null;
    currentProgramSceneName: string | null;
    scenes: Scene[];
}

export type { Scene, Scenes };
