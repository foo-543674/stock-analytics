import styles from './Header.module.css';
import { AiOutlineStock } from 'solid-icons/ai';
import { AiOutlineMenu } from 'solid-icons/ai';
import { PrimaryButton } from './PrimaryButton';

type HeaderProps = Partial<{
  onToggleMenu: () => void;
}>;

export const Header = (props: HeaderProps) => {
  return (
    <header class={styles.header}>
      <div class={styles.container}>
        <PrimaryButton
          class={styles.mobileMenuBtn}
          onClick={props.onToggleMenu}
          aria-label="Toggle menu"
        >
          <AiOutlineMenu class={styles.menuIcon} />
        </PrimaryButton>
        <a href="/" class={styles.logo}>
          <AiOutlineStock class={styles.logoIcon} />
          <span class={styles.logoText}>Stock analytics</span>
        </a>
      </div>
    </header>
  );
};
