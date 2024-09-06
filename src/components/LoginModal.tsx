import { forwardRef } from "react";
import Modal from "./Modal";
import { open } from "@tauri-apps/api/shell";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/tauri";

interface Props {
    isOpened: boolean;
}

const LoginModal = forwardRef(function LoginModal({ isOpened }: Props, ref) {
    const { t } = useTranslation();
    // const oauthUrl =
    //     "https://id.twitch.tv/oauth2/authorize?client_id=cig4pc07b7bxo207x8158v58r1i5pf&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8457&scope=channel%3Amanage%3Apredictions";

    const oauthUrl = new URL("https://id.twitch.tv/oauth2/authorize");
    const params = new URLSearchParams(oauthUrl.search);
    params.set("client_id", "cig4pc07b7bxo207x8158v58r1i5pf");
    params.set("response_type", "code");
    params.set("redirect_uri", "http://localhost:8457");
    params.set(
        "scope",
        "channel:manage:predictions moderator:manage:announcements user:read:chat user:write:chat"
    );
    const oauthUrlString = `${oauthUrl.origin + oauthUrl.pathname}?${params}`;

    const handleClick = () => {
        open(oauthUrlString);
        invoke("start_login_flow");
    };

    return (
        <Modal ref={ref} isOpened={isOpened} canBeClosed={false}>
            <h2>{t("Welcome to Flow")}</h2>
            <p>
                {t(
                    "Thanks for downloading Flow, if you have any questions or suggestions, please contact me on Twitter. If you need help to set up Flow, you can find help on the Discord."
                )}
            </p>
            <button onClick={handleClick} type="button">
                {t("Login with Twitch")}
            </button>
        </Modal>
    );
});

export default LoginModal;
