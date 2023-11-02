import { forwardRef } from "react";
import Modal from "./Modal";

const BotModalWizeBot = forwardRef(function BotModalWizeBot(_, ref) {
    return (
        <Modal ref={ref} canBeClosed={true}>
            <h2>WizeBot</h2>
            <p>
                Pour vous offrir une expérience optimale, nous avons besoin de
                votre autorisation pour accéder à votre compte Twitch. Cette
                autorisation nous permettra de récupérer des informations de
                base de votre profil et d'interagir avec votre compte
                conformément à nos services. Votre vie privée est notre
                priorité, et nous nous engageons à maintenir vos informations en
                toute sécurité. En cliquant sur le bouton "Autoriser"
                ci-dessous, vous acceptez de nous accorder l'accès nécessaire
                pour continuer.
            </p>
        </Modal>
    );
});

export default BotModalWizeBot;
