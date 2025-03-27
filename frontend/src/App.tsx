import { PageRouter } from '@/routes/PageRouter';
import './index.css';
import { createTheme, themeNames } from '@/primitives/createTheme';
import { TranslateProvider } from './contexts/TranslateContext';
import { ApiClientProvider } from './contexts/ApiClientContext';
import { PageFrame } from './pages/frame/PageFrame';

const baseUrl = 'http://localhost:5173';

export const App = () => {
  const [theme, toggleTheme] = createTheme();
  const actualTheme = () => themeNames[theme()];

  return (
    <div data-theme={actualTheme()}>
      <ApiClientProvider baseUrl={baseUrl}>
        <TranslateProvider baseUrl={baseUrl}>
          <PageFrame theme={theme()} toggleTheme={toggleTheme}>
            <PageRouter />
          </PageFrame>
        </TranslateProvider>
      </ApiClientProvider>
    </div>
  );
};
