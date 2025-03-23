import { ApiClient, ApiResponse } from '@/api/ApiClient';
import { describe, it, expect, vi, afterEach } from 'vitest';
import { createBrandsList } from './createBrandsList';
import { brands } from '@tests/mocks/brands';
import { renderHook } from '@solidjs/testing-library';
import { waitMockResolved } from '@tests/waitMockResolved';

const mockedFetching = vi.fn().mockImplementation(<
  T,
>(): Promise<ApiResponse<T>> => {
  const result: ApiResponse<T> = {
    _tag: 'Right',
    value: { page: 1, maxPage: 2, items: brands },
  } as unknown as ApiResponse<T>;
  return Promise.resolve(result);
});

const apiClientMock: ApiClient = {
  get: mockedFetching,
  post: vi.fn(),
  put: vi.fn(),
  delete: vi.fn(),
};

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
    expect(mockedFetching).toHaveBeenCalledTimes(1);
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
    expect(mockedFetching).toHaveBeenCalledTimes(1);
  });

  it('should not refetch automaticly when filter changed', async () => {
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();
    expect(result().filter).toEqual({});
    result().setFilter({ brandName: 'test' });
    expect(result().filter).toEqual({ brandName: 'test' });
    expect(mockedFetching).toHaveBeenCalledTimes(1);
  });

  it('should refetch when refetch called', async () => {
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();
    result().setPage(2);
    result().refetch();
    expect(mockedFetching).toHaveBeenCalledTimes(2);
  });
});
