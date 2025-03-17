import { createSignal } from 'solid-js';

export type Theme = 'light' | 'dark';

//NOTE: Specify theme name by 'light' or 'dark' is not working on below code. I don't know why.
//@plugin "daisyui" {
//   themes: corporate --default, business --dark;
//}
export const themeNames = {
  light: 'corporate',
  dark: 'business',
};

export const createTheme = (): [() => Theme, () => void] => {
  const [theme, setTheme] = createSignal<Theme>('light');

  const toggleTheme = () => {
    setTheme(theme() === 'light' ? 'dark' : 'light');
  };

  return [theme, toggleTheme];
};
