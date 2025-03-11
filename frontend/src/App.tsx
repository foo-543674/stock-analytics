import { Header } from '@/components/Header';
import { createSignal } from 'solid-js';
import { PageRouter } from './routes/PageRouter';

export const App = () => {
  const [opened, setOpened] = createSignal(false);

  return (
    <>
      <Header isMenuOpened={opened()} onMenuOpenChanged={setOpened}></Header>
      <div class="pt-16">
        <PageRouter />
      </div>
    </>
  );
};
