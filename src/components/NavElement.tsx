import { Link } from "react-router-dom";

interface Props {
    title: string;
    link: string;
}

function NavElement({ title, link }: Props) {
    return (
        <>
            <li>
                <Link to={link}>
                    <h3>{title}</h3>
                </Link>
            </li>
        </>
    );
}

export default NavElement;
