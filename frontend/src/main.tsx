import { render } from 'solid-js/web';
import { App } from './App';

const rootElement = document.getElementById('root');
if (rootElement) {
  render(() => <App apiUrl={import.meta.env.VITE_API_URL} />, rootElement);
} else {
  console.error('Root element not found');
}
