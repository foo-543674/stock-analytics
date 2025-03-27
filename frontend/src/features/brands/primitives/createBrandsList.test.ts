import { describe, it, expect, vi, afterEach, beforeAll } from 'vitest';
import { createBrandsList } from './createBrandsList';
import { brands } from '@tests/mocks/brands';
import { renderHook } from '@solidjs/testing-library';
import { waitMockResolved } from '@tests/waitMockResolved';
import { createApiClientMock } from '@tests/mocks/ApiClientMock';
import { fetchBrands } from '@/features/brands/api/fetchBrands';
import { ok } from '@/utils/Result';
import { pageStub } from '@tests/mocks/pageStub';
import {
  errorResponseStub,
  neverReturningResponseImpl,
} from '@tests/mockHttpResult';

describe('createBrandsList', () => {
  const [apiClientMock] = createApiClientMock();
  beforeAll(() => {
    vi.mock('@/data-access/api/brands/fetchBrands');
  });
  afterEach(() => {
    vi.clearAllMocks();
  });

  it('should fetch brands page', async () => {
    vi.mocked(fetchBrands).mockResolvedValue(ok(pageStub(brands, 1, 2)));
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();

    expect(result().isLoading).toBe(false);
    expect(result().page).toBe(1);
    expect(result().maxPage).toBe(2);
    expect(result().brands).toBe(brands);
  });

  it('should be loading while fetching', async () => {
    vi.mocked(fetchBrands).mockImplementation(neverReturningResponseImpl);
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    expect(result().isLoading).toBe(true);
  });

  it('should not refetch automaticly when page changed', async () => {
    vi.mocked(fetchBrands).mockResolvedValue(ok(pageStub(brands, 1, 2)));
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();
    expect(result().page).toBe(1);
    result().setPage(2);
    expect(result().page).toBe(2);
    expect(vi.mocked(fetchBrands)).toHaveBeenCalledTimes(1);
  });

  it('should not refetch automaticly when filter changed', async () => {
    vi.mocked(fetchBrands).mockResolvedValue(ok(pageStub(brands, 1, 2)));
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();
    expect(result().filter).toEqual({});
    result().setFilter({ brandName: 'test' });
    expect(result().filter).toEqual({ brandName: 'test' });
    expect(vi.mocked(fetchBrands)).toHaveBeenCalledTimes(1);
  });

  it('should refetch when refetch called', async () => {
    vi.mocked(fetchBrands).mockResolvedValue(ok(pageStub(brands, 1, 2)));
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();
    result().setPage(2);
    result().refetch();
    expect(vi.mocked(fetchBrands)).toHaveBeenCalledTimes(2);
  });

  it('should be error when fetch failed', async () => {
    vi.mocked(fetchBrands).mockResolvedValue(errorResponseStub);
    const { result } = renderHook(() => createBrandsList(apiClientMock));

    await waitMockResolved();
    expect(result().isError).toBe(true);
  });
});
