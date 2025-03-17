import { describe, expect, it } from 'vitest';
import { createTheme } from './createTheme';

describe('createTheme', () => {
  it('should return light by default', async () => {
    const [theme] = createTheme();
    expect(theme()).toBe('light');
  });

  it('should return dark after toggling', async () => {
    const [theme, toggleTheme] = createTheme();
    toggleTheme();
    expect(theme()).toBe('dark');
  });

  it('should return light after toggling twice', async () => {
    const [theme, toggleTheme] = createTheme();
    toggleTheme();
    toggleTheme();
    expect(theme()).toBe('light');
  });
});
