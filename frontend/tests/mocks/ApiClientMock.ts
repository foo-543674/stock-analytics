import { ApiClient } from '@/data-access/api/ApiClient';
import { Mock, vi } from 'vitest';

export const createApiClientMock = <
  T extends Partial<Record<keyof ApiClient, Mock>>,
>(
  methods: T,
): [ApiClient, T] => {
  const client: ApiClient = {
    get: methods.get ?? vi.fn(),
    post: methods.post ?? vi.fn(),
    put: methods.put ?? vi.fn(),
    delete: methods.delete ?? vi.fn(),
  };
  return [client, methods] as const;
};
