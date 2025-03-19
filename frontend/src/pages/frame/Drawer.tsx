import { Translate } from '@/primitives/createTranslate';
import { routes } from '@/routes/routes';
import { AiOutlineArrowLeft } from 'solid-icons/ai';
import { JSX } from 'solid-js';

type DrawerProps = Partial<{
  isMenuOpened: boolean;
  onMenuOpenChanged: (isOpen: boolean) => void;
  translate: Translate;
  children: JSX.Element;
}>;

export const Drawer = (props: DrawerProps) => {
  return (
    <div class="drawer lg:drawer-open">
      <input
        type="checkbox"
        class="drawer-toggle"
        checked={props.isMenuOpened}
      />
      <div class="drawer-content flex">{props.children}</div>
      <div class="drawer-side top-0 z-50 lg:z-20">
        <label
          data-testid="drawer-overlay"
          class="drawer-overlay"
          onClick={() => props.onMenuOpenChanged?.(false)}
        />
        <ul class="menu menu-md bg-base-200 min-h-full w-64 text-base-content">
          <button
            data-testid="drawer-close-button"
            class="btn btn-circle btn-ghost drawer-button lg:hidden"
            onClick={() => props.onMenuOpenChanged?.(false)}
          >
            <AiOutlineArrowLeft />
          </button>
          <li class="menu-title text-lg">Foo</li>
          <li>
            <a href={routes.brands}>{props.translate?.('brandPageLink')}</a>
          </li>
        </ul>
      </div>
    </div>
  );
};
