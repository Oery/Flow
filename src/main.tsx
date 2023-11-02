import ReactDOM from "react-dom/client";
import Root from "./routes/root";
import ErrorPage from "./routes/error-page";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import SettingsPage from "./routes/SettingsPage";
import ScenePage from "./routes/ScenePage";
import HomePage from "./routes/HomePage";
import PackPage from "./routes/PackPage";
import MusicPage from "./routes/MusicPage";
import ServerPage from "./routes/ServerPage";

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
        ],
    },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <RouterProvider router={router} />
);
