import { beforeAll, describe, expect, vi } from 'vitest';
import { createTranslate } from './createTranslate';
import { fetchLocales } from '../data-access/fetchLocales';
import { waitMockResolved } from 'tests/waitMockResolved';

describe('createTranslate', test => {
  beforeAll(() => {
    vi.mock('../data-access/fetchLocales');
  });

  test('should return a function that get text for key', async () => {
    vi.mocked(fetchLocales).mockResolvedValue({ key: 'value' });

    const translate = createTranslate('en');
    await waitMockResolved();
    const result = translate('key')();
    expect(result).toBe('value');
  });

  test('should return the key if the key is not found', async () => {
    vi.mocked(fetchLocales).mockResolvedValue({});

    const translate = createTranslate('en');
    await waitMockResolved();
    const result = translate('key')();
    expect(result).toBe('key');
  });

  test('should return empty string if locales is loading', async () => {
    vi.mocked(fetchLocales).mockImplementation(() => new Promise(() => {}));

    const translate = createTranslate('en');
    const result = translate('key')();
    expect(result).toBe('');
  });

  test('should return empty string if locales is error', async () => {
    vi.mocked(fetchLocales).mockRejectedValue('error');

    const translate = createTranslate('en');
    await waitMockResolved();
    const result = translate('key')();
    expect(result).toBe('');
  });
});
