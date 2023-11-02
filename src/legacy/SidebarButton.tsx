interface Props {
    icon: string;
    text: string;
}

function SidebarButton({ icon, text }: Props) {
    return (
        <a href="">
            <span className="material-icons-outlined">{icon}</span>
            <p>{text}</p>
        </a>
    );
}

export default SidebarButton;
