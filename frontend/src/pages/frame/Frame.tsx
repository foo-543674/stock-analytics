import { Translate } from '@/primitives/createTranslate';
import { Drawer } from './Drawer';
import { JSX } from 'solid-js';
import { Navbar } from './Navbar';
import { Theme } from '@/primitives/createTheme';

type FrameProps = Partial<{
  isMenuOpened: boolean;
  theme: Theme;
  onMenuOpenChanged: (isOpen: boolean) => void;
  toggleTheme: () => void;
  translate: Translate;
  children: JSX.Element;
}>;
export const Frame = (props: FrameProps) => {
  return (
    <div>
      <Navbar
        isMenuOpened={props.isMenuOpened}
        theme={props.theme}
        onMenuOpenChanged={props.onMenuOpenChanged}
        toggleTheme={props.toggleTheme}
        translate={props.translate}
      />
      <Drawer
        isMenuOpened={props.isMenuOpened}
        onMenuOpenChanged={props.onMenuOpenChanged}
        translate={props.translate}
      >
        <div class="bg-base-100 w-full h-full p-4">{props.children}</div>
      </Drawer>
    </div>
  );
};
