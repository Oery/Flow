import { Link } from "react-router-dom";

interface Props {
    title: string;
    link: string;
    handleClick?: () => void;
}

function NavElement({ title, link, handleClick }: Props) {
    return (
        <li onClick={handleClick}>
            <Link to={link}>
                <h3>{title}</h3>
            </Link>
        </li>
    );
}

export default NavElement;
