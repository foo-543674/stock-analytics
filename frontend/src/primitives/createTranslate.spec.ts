import { beforeAll, describe, expect, vi } from 'vitest';
import { createTranslate } from './createTranslate';
import { fetchLocales } from '../data-access/fetchLocales';
import { waitMockResolved } from 'tests/waitMockResolved';
import { renderHook } from '@solidjs/testing-library';

describe('createTranslate', test => {
  beforeAll(() => {
    vi.mock('../data-access/fetchLocales');
  });

  test('should return a function that get text for key', async () => {
    vi.mocked(fetchLocales).mockResolvedValue({ key: 'value' });

    const { result: translate } = renderHook(() => createTranslate('en'));
    await waitMockResolved();
    const result = translate()('key');
    expect(result).toBe('value');
  });

  test('should return the key if the key is not found', async () => {
    vi.mocked(fetchLocales).mockResolvedValue({});

    const { result: translate } = renderHook(() => createTranslate('en'));
    await waitMockResolved();
    const result = translate()('key');
    expect(result).toBe('key');
  });

  test('should return empty string if locales is loading', async () => {
    vi.mocked(fetchLocales).mockImplementation(() => new Promise(() => {}));

    const { result: translate } = renderHook(() => createTranslate('en'));
    const result = translate()('key');
    expect(result).toBe('');
  });

  test('should return empty string if locales is error', async () => {
    vi.mocked(fetchLocales).mockRejectedValue('error');

    const { result: translate } = renderHook(() => createTranslate('en'));
    await waitMockResolved();
    const result = translate()('key');
    expect(result).toBe('');
  });
});
