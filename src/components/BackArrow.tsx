import { Link } from "react-router-dom";
import styles from "../styles/BackArrow.module.css";

interface Props {
    destination: string;
}

export default function BackArrow({ destination }: Props) {
    return (
        <Link to={destination} className={styles.backarrow}>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20.018"
                height="22.5"
                viewBox="0 0 20.018 22.5"
            >
                <path
                    id="Polygone_1"
                    data-name="Polygone 1"
                    d="M10.378,1.551a1,1,0,0,1,1.744,0l9.541,16.976a1,1,0,0,1-.872,1.49H1.709a1,1,0,0,1-.872-1.49Z"
                    transform="translate(0 22.5) rotate(-90)"
                    fill="#888"
                />
            </svg>
        </Link>
    );
}
