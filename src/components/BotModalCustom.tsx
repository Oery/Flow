import { forwardRef } from "react";
import Modal from "./Modal";

const BotModalCustom = forwardRef(function BotModalCustom(_, ref) {
    return (
        <Modal ref={ref} canBeClosed={true}>
            <h2>Custom</h2>
            <p>Salutations, à tous et à toutes !</p>
        </Modal>
    );
});

export default BotModalCustom;
