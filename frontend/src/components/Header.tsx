import { Translate } from '@/primitives/createTranslate';
import styles from './Header.module.css';
import {
  AiOutlineArrowLeft,
  AiOutlineMenu,
  AiOutlineStock,
} from 'solid-icons/ai';
import { routes } from '@/routes/routes';
import { Button } from './Button';
import { Show } from 'solid-js';

type HeaderProps = Partial<{
  isMenuOpened: boolean;
  onMenuOpenChanged: (opened: boolean) => void;
  translate: Translate;
}>;

export const Header = (props: HeaderProps) => {
  return (
    <div class={styles.header}>
      <div class={styles.container}>
        <Button
          class={`${styles.openBtn}`}
          data-testid="openBtn"
          onClick={() => props.onMenuOpenChanged?.(true)}
          aria-label="Open menu"
        >
          <AiOutlineMenu class={styles.btnIcon} />
        </Button>
        <a href="/" class={styles.logo}>
          <AiOutlineStock class={styles.logoIcon} />
          <span class={styles.logoText}>
            {props.translate?.('serviceName')}
          </span>
        </a>
        <div
          class={`${styles.nav} ${props.isMenuOpened ? styles.opened : styles.closed}`}
        >
          <Button
            class={`${styles.closeBtn}`}
            data-testid="closeBtn"
            onClick={() => props.onMenuOpenChanged?.(false)}
            aria-label="Close menu"
          >
            <AiOutlineArrowLeft class={styles.toggleIcon} />
          </Button>
          <a
            href={routes.brands}
            class={`${styles.navItem} ${styles.clickable}`}
          >
            {props.translate?.('brandPageLink')}
          </a>
        </div>
        <Show when={props.isMenuOpened}>
          <div
            class={styles.overlay}
            onClick={() => props.onMenuOpenChanged?.(false)}
          ></div>
        </Show>
      </div>
    </div>
  );
};
