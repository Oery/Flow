import i18n from "i18next";
import LanguageDetector from "i18next-browser-languagedetector";
import { initReactI18next } from "react-i18next";

const resources = {
    fr: {
        translation: {
            Server: "Serveur",
            Home: "Accueil",
            Scene: "Scène",
            Pack: "Pack",
            Music: "Musique",
            Settings: "Options",
            Exit: "Quitter",
            Status: "Statut",
            "Queuing Scene": "Scène de Queue",
            "Server Address": "IP du serveur",
            "Minecraft Client": "Client Minecraft",
            "Twitch Bot": "Bot Twitch",
            "Disable Hardware Acceleration": "Désactiver l'accélération matérielle",
            "Start with Windows": "Démarrer avec Windows",
            "Streaming Only": "En stream uniquement",
            "Only in Minecraft category": "Uniquement dans la catégorie Minecraft",
            "Command Text": "Réponse de la commande",
            "Hide Overlay": "Cacher les overlays",
            Announce: "Annoncer le changement",
            "Announce Text": "Texte de l'annonce",
            "Please connect Flow to OBS to access your scenes":
                "Veuillez connecter Flow à OBS pour accéder à vos scènes",
            "Toggle Scene after Game End": "Cacher l'écran à la fin d'une partie",
            "Auto OBS Configuration": "Configuration automatique du Websocket OBS",
            "Websocket Host": "Hôte du Websocket",
            "Websocket Password": "Mot de passe",
            "Keep Screen hidden in Lobby": "Cacher l'écran dans le lobby",
            "Queuing Scene Name": "Nom de la scène de Queue",
            Online: "Connecté",
            Bad: "Hors ligne",
            Offline: "Hors ligne",
            "Ignore Twitch": "Ignorer Twitch",
            Unknown: "Inconnu",
            "In Game": "En jeu",
            InGame: "En jeu",
            "Welcome to Flow": "Bienvenue",
            "Thanks for downloading Flow, if you have any questions or suggestions, please contact me on Twitter. If you need help to set up Flow, you can find help on the Discord.":
                "Merci d'avoir téléchargé Flow, si vous avez une question ou une suggestion, veuillez me contacter sur Twitter. Si vous avez besoin d'aide pour configurer Flow, vous pouvez trouver de l'aide sur le Discord.",
            "Login with Twitch": "Se connecter avec Twitch",
            "Connect to Nightbot": "Se connecter à Nightbot",
            Language: "Langue",
            "Your Packs": "Vos packs",
        },
    },
};

i18n.use(initReactI18next)
    .use(LanguageDetector)
    .init({
        resources,
        fallbackLng: "en",

        interpolation: {
            escapeValue: false,
        },
    });

export default i18n;
