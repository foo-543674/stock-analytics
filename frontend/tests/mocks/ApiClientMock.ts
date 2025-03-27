import { ApiClient } from '@/data-access/ApiClient';
import { Mock, vi } from 'vitest';

export const createApiClientMock = <
  T extends Partial<Record<keyof ApiClient, Mock>>,
>(
  methods: T = {} as T,
): [ApiClient, T] => {
  const client: ApiClient = {
    get: methods.get ?? vi.fn(),
  };
  return [client, methods] as const;
};
