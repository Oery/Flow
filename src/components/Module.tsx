import { type PropsWithChildren, Children } from "react";
import styles from "../styles/Module.module.css";

interface Props {
    title: string | string[];
    column?: boolean;
}

function Module({ title, children, column }: PropsWithChildren<Props>) {
    if (typeof title === "object") {
        if (title.length !== Children.count(children)) {
            console.error("The number of titles and children should be the same.");
            return null;
        }

        return (
            <div className={styles.module}>
                {title.map((text, index) => {
                    return (
                        <div key={text} className={`${column ? styles.column : ""}`}>
                            <h3>{text}</h3>
                            {Children.toArray(children)[index]}
                        </div>
                    );
                })}
            </div>
        );
    }

    return (
        <div className={`${styles.module} ${column ? styles.column : ""}`}>
            <h3>{title}</h3>
            {children}
        </div>
    );
}

export default Module;
