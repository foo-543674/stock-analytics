import { Theme } from '@/primitives/createTheme';
import { Translate } from '@/primitives/createTranslate';
import { AiOutlineMenu } from 'solid-icons/ai';
import { FiMoon, FiSun } from 'solid-icons/fi';

type NavbarProps = Partial<{
  isMenuOpened: boolean;
  theme: Theme;
  onMenuOpenChanged: (isOpen: boolean) => void;
  toggleTheme: () => void;
  translate: Translate;
}>;

export const Navbar = (props: NavbarProps) => {
  const isLight = () => (props.theme ?? 'light') === 'light';

  return (
    <header class="navbar sticky top-0 z-50 bg-primary shadow-lg">
      <div class="navbar-start">
        <button
          data-testid="navbar-menu-button"
          class="btn btn-circle btn-primary drawer-button lg:hidden"
          onClick={() => props.onMenuOpenChanged?.(!props.isMenuOpened)}
        >
          <AiOutlineMenu class="w-6 h-6" />
        </button>
        <a href="/" class="text-xl text-primary-content text-nowrap lg:ml-4">
          {props.translate?.('serviceName')}
        </a>
      </div>
      <div class="navbar-end">
        <button
          data-testid="navbar-theme-button"
          class="btn btn-primary btn-circle w-10 h-10 swap swap-rotate"
          data-toggle-theme="dark,light"
          data-act-class="ACTIVECLASS"
          onClick={() => props.toggleTheme?.()}
        >
          <input type="checkbox" checked={isLight()} />
          <FiSun class="swap-on w-6 h-6" />
          <FiMoon class="swap-off w-6 h-6" />
        </button>
      </div>
    </header>
  );
};
