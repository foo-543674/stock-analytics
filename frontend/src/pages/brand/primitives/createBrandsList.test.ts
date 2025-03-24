import { ApiResponse } from '@/api/ApiClient';
import { describe, it, expect, vi, afterEach } from 'vitest';
import { createBrandsList } from './createBrandsList';
import { brands } from '@tests/mocks/brands';
import { renderHook } from '@solidjs/testing-library';
import { waitMockResolved } from '@tests/waitMockResolved';
import { left, right } from '@/utils/Either';
import { createApiClientMock } from '@tests/mocks/ApiClientMock';

const [apiClientMock, mocked] = createApiClientMock({
  get: vi.fn().mockImplementation(<T>(): Promise<ApiResponse<T>> => {
    const result: ApiResponse<T> = right({
      page: 1,
      maxPage: 2,
      items: brands,
    }) as unknown as ApiResponse<T>;
    return Promise.resolve(result);
  }),
});

describe('createBrandsList', () => {
  afterEach(() => {
    vi.clearAllMocks();
  });

  it('should fetch brands page', async () => {
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();

    expect(result().page).toBe(1);
    expect(result().maxPage).toBe(2);
    expect(result().brands).toBe(brands);
    expect(mocked.get).toHaveBeenCalledTimes(1);
  });

  it('should be loading while fetching', async () => {
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    expect(result().isLoading).toBe(true);
    await waitMockResolved();
    expect(result().isLoading).toBe(false);
  });

  it('should not refetch automaticly when page changed', async () => {
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();
    expect(result().page).toBe(1);
    result().setPage(2);
    expect(result().page).toBe(2);
    expect(mocked.get).toHaveBeenCalledTimes(1);
  });

  it('should not refetch automaticly when filter changed', async () => {
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();
    expect(result().filter).toEqual({});
    result().setFilter({ brandName: 'test' });
    expect(result().filter).toEqual({ brandName: 'test' });
    expect(mocked.get).toHaveBeenCalledTimes(1);
  });

  it('should refetch when refetch called', async () => {
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();
    result().setPage(2);
    result().refetch();
    expect(mocked.get).toHaveBeenCalledTimes(2);
  });

  it('should be error when fetch failed', async () => {
    const [apiClientMock] = createApiClientMock({
      get: vi.fn().mockImplementation(<T>(): Promise<ApiResponse<T>> => {
        const result: ApiResponse<T> = left({
          message: 'error',
          status: 500,
        }) as unknown as ApiResponse<T>;
        return Promise.resolve(result);
      }),
    });

    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();
    expect(result().isError).toBe(true);
  });

  it('should be error when request send failed', async () => {
    const [apiClientMock] = createApiClientMock({
      get: vi.fn().mockImplementation(<T>(): Promise<ApiResponse<T>> => {
        return Promise.reject(new Error('error'));
      }),
    });
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();
    expect(result().isError).toBe(true);
  });
});
