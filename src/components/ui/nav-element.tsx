import { Link } from 'react-router-dom';

interface Props {
    title: string;
    link: string;
    handleClick?: () => void;
}

function NavElement({ title, link, handleClick }: Props) {
    return (
        // biome-ignore lint/a11y/useKeyWithClickEvents: <explanation>
        <li onClick={handleClick}>
            <Link to={link}>
                <h3 className='text-2xl leading-[35px]  text-flow-primary'>{title}</h3>
            </Link>
        </li>
    );
}

export default NavElement;
