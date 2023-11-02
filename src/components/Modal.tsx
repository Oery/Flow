import Titlebar from "./TitleBar";
import styles from "../styles/Modal.module.css";
import { forwardRef, useEffect, useState } from "react";

const Modal = forwardRef(function Modal(
    { children, isOpened, canBeClosed }: any,
    ref: any
) {
    const [hidden, setHidden] = useState(false);

    useEffect(() => {
        if (isOpened) {
            ref.current.showModal();
            return;
        }

        setHidden(true);
        setTimeout(() => {
            ref.current.close();
            setHidden(false);
            console.log("closed modal");
        }, 300);
    }, [isOpened]);

    const handleClick = (e: any) => {
        if (e.target.nodeName !== "DIALOG") {
            return;
        }

        if (!canBeClosed) {
            return;
        }

        const dialogDimensions =
            ref.current.childNodes[1].childNodes[0].getBoundingClientRect();
        if (
            e.clientX < dialogDimensions.left ||
            e.clientX > dialogDimensions.right ||
            e.clientY < dialogDimensions.top ||
            e.clientY > dialogDimensions.bottom
        ) {
            setHidden(true);
            setTimeout(() => {
                e.target?.close();
                setHidden(false);
            }, 300);
        }
    };

    return (
        <dialog
            ref={ref}
            className={`${styles.modal} ${hidden ? styles.hidden : ""}`}
            onClick={handleClick}
            id="dialog"
        >
            <Titlebar />
            <div className={styles.container} id="dialog-container">
                <div className={styles.content}>{children}</div>
            </div>
        </dialog>
    );
});

export default Modal;
