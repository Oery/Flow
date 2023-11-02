import { forwardRef } from "react";
import Modal from "./Modal";

const BotModalNightbot = forwardRef(function BotModalNightbot(_, ref) {
    return (
        <Modal ref={ref} canBeClosed={true}>
            <h2>Nightbot</h2>
            <p>Salutations, à tous et à toutes !</p>
        </Modal>
    );
});

export default BotModalNightbot;
