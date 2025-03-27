import { beforeAll, describe, expect, vi } from 'vitest';
import { fetchLocales } from '@/data-access/fetchLocales';
import { waitMockResolved } from 'tests/waitMockResolved';
import { renderHook } from '@solidjs/testing-library';
import { createTranslate } from './createTranslate';
import { ok } from '@/utils/Result';
import {
  errorResponseStub,
  neverReturningResponseImpl,
} from '@tests/mockHttpResult';

describe('createTranslate', test => {
  beforeAll(() => {
    vi.mock('@/data-access/fetchLocales');
  });

  test('should return a function that get text for key', async () => {
    vi.mocked(fetchLocales).mockResolvedValue(ok({ key: 'value' }));

    const { result: translate } = renderHook(() => createTranslate('en'));
    await waitMockResolved();
    const result = translate()('key');
    expect(result).toBe('value');
  });

  test('should return the key if the key is not found', async () => {
    vi.mocked(fetchLocales).mockResolvedValue(ok({}));

    const { result: translate } = renderHook(() => createTranslate('en'));
    await waitMockResolved();
    const result = translate()('key');
    expect(result).toBe('key');
  });

  test('should return empty string if locales is loading', async () => {
    vi.mocked(fetchLocales).mockImplementation(neverReturningResponseImpl);

    const { result: translate } = renderHook(() => createTranslate('en'));
    const result = translate()('key');
    expect(result).toBe('');
  });

  test('should return empty string if locales is error', async () => {
    vi.mocked(fetchLocales).mockResolvedValue(errorResponseStub);

    const { result: translate } = renderHook(() => createTranslate('en'));
    await waitMockResolved();
    const result = translate()('key');
    expect(result).toBe('');
  });

  test('should replace the placeholders with the replacements', async () => {
    vi.mocked(fetchLocales).mockResolvedValue(
      ok({ key: 'Hello, {{ name }}!' }),
    );

    const { result: translate } = renderHook(() => createTranslate('en'));
    await waitMockResolved();
    const result = translate()('key', { name: 'world' });
    expect(result).toBe('Hello, world!');
  });
});
