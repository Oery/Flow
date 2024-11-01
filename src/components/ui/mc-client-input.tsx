import styles from '../styles/SelectInput.module.css';

export default function MCClientInput() {
    return (
        <select className={styles.selectinput}>
            <option value='Vanilla / Forge'>Vanilla / Forge</option>
            <option value='Badlion Client'>Badlion Client</option>
            <option value='Lunar Client'>Lunar Client</option>
            <option value='Feather Client'>Feather Client</option>
            <option value='Custom'>Custom</option>
        </select>
    );
}
