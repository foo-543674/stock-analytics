import { Header } from '@/components/Header';
import { createSignal } from 'solid-js';
import { PageRouter } from './routes/PageRouter';
import { createTranslate } from './primitives/createTranslate';
import { createBrowserConfig } from './primitives/createBrowserConfig';

export const App = () => {
  const [opened, setOpened] = createSignal(false);
  const browserConfig = createBrowserConfig();
  const translate = createTranslate(browserConfig.lang);

  return (
    <>
      <Header
        isMenuOpened={opened()}
        onMenuOpenChanged={setOpened}
        translate={translate}
      ></Header>
      <div class="pt-16">
        <PageRouter />
      </div>
    </>
  );
};
