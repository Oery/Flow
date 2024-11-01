import ReactDOM from "react-dom/client";
import Root from "./pages/root";
import ErrorPage from "./pages/error-page";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import SettingsPage from "./pages/settings-page";
import ScenePage from "./pages/scene-page";
import HomePage from "./pages/home-page";
import PackPage from "./pages/pack-page";
import MusicPage from "./pages/music-page";
import ServerPage from "./pages/server-page";
import BotPage from "./pages/bot-page";

import "./i18n";

const router = createBrowserRouter([
    {
        path: "/",
        element: <Root />,
        errorElement: <ErrorPage />,

        children: [
            { element: <HomePage />, index: true },
            { path: "settings", element: <SettingsPage /> },
            { path: "scene", element: <ScenePage /> },
            { path: "dynpack", element: <PackPage /> },
            { path: "dynip", element: <ServerPage /> },
            { path: "music", element: <MusicPage /> },
            { path: "bot", element: <BotPage /> },
        ],
    },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <RouterProvider router={router} />
);
