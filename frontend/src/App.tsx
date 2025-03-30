import { PageRouter } from '@/routes/PageRouter';
import './index.css';
import { createTheme, themeNames } from '@/primitives/createTheme';
import { TranslateProvider } from './contexts/TranslateContext';
import { ApiClientProvider } from './contexts/ApiClientContext';
import { PageFrame } from './pages/frame/PageFrame';

export type AppProps = Partial<{
  apiUrl?: string;
}>;

export const App = (props: AppProps) => {
  const [theme, toggleTheme] = createTheme();
  const actualTheme = () => themeNames[theme()];

  const apiUrl = () => props.apiUrl || '/api';

  return (
    <div data-theme={actualTheme()}>
      <ApiClientProvider baseUrl={apiUrl()}>
        <TranslateProvider>
          <PageFrame theme={theme()} toggleTheme={toggleTheme}>
            <PageRouter />
          </PageFrame>
        </TranslateProvider>
      </ApiClientProvider>
    </div>
  );
};
