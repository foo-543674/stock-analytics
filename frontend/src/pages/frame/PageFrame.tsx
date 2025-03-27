import { Theme } from '@/primitives/createTheme';
import { createSignal, JSX, Show, useContext } from 'solid-js';
import { Frame } from './Frame';
import { TranslateContext } from '@/contexts/TranslateContext';
import { Loading } from '@/components/Loading';

export type PageFrameProps = {
  theme: Theme;
  toggleTheme: () => void;
  children: JSX.Element;
};

export const PageFrame = (props: PageFrameProps) => {
  const [opened, setOpened] = createSignal(false);
  const translate = useContext(TranslateContext);
  return (
    <Show when={translate} fallback={<Loading />}>
      {t => (
        <Frame
          isMenuOpened={opened()}
          theme={props.theme}
          onMenuOpenChanged={setOpened}
          toggleTheme={props.toggleTheme}
          translate={t()()}
        >
          {props.children}
        </Frame>
      )}
    </Show>
  );
};
