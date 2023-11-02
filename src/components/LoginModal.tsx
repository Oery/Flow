import { forwardRef } from "react";
import Modal from "./Modal";
import { open } from "@tauri-apps/api/shell";

interface Props {
    isOpened: boolean;
}

const LoginModal = forwardRef(function LoginModal({ isOpened }: Props, ref) {
    const oauthUrl =
        "https://id.twitch.tv/oauth2/authorize?client_id=cig4pc07b7bxo207x8158v58r1i5pf&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8457&scope=channel%3Amanage%3Apredictions";

    return (
        <Modal ref={ref} isOpened={isOpened} canBeClosed={false}>
            <h2>Login</h2>
            <p>Salutations, à tous et à toutes !</p>
            <button onClick={() => open(oauthUrl)}>
                CLICK HERE ON CONNECT
            </button>
        </Modal>
    );
});

export default LoginModal;
