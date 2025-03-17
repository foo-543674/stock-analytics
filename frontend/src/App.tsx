import { createSignal } from 'solid-js';
import { PageRouter } from '@/routes/PageRouter';
import { createTranslate } from '@/primitives/createTranslate';
import { createBrowserConfig } from '@/primitives/createBrowserConfig';
import { Frame } from '@/pages/frame/Frame';
import './index.css';
import { createTheme, themeNames } from '@/primitives/createTheme';

export const App = () => {
  const [opened, setOpened] = createSignal(false);
  const browserConfig = createBrowserConfig();
  const translate = createTranslate(browserConfig.lang);
  const [theme, toggleTheme] = createTheme();

  const actualTheme = () => themeNames[theme()];

  return (
    <div data-theme={actualTheme()}>
      <Frame
        isMenuOpened={opened()}
        theme={theme()}
        onMenuOpenChanged={setOpened}
        toggleTheme={toggleTheme}
        translate={translate()}
      >
        <PageRouter />
      </Frame>
    </div>
  );
};
