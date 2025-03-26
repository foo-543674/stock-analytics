import { describe, it, expect, afterEach, beforeAll, afterAll } from 'vitest';
import { getRequest } from './http';
import ky from 'ky';
import { ok } from '@/utils/Result';
import { parseError } from '@/schemas/ParseResult';
import { notFound } from './HttpError';
import { setupServer } from 'msw/node';
import { http, HttpResponse } from 'msw';

describe('http', () => {
  const server = setupServer();
  beforeAll(() => {
    server.listen();
  });
  afterAll(() => {
    server.close();
  });
  afterEach(() => {
    server.resetHandlers();
  });

  const url = 'https://example.com/api/v1/foo';

  describe('get', () => {
    it('should fetch data', async () => {
      const bodyStub = { foo: 'bar' };
      type Body = typeof bodyStub;
      server.use(
        http.get(url, () => HttpResponse.json(bodyStub, { status: 200 })),
      );
      const response = await getRequest(ky)<Body>(url, {}, data =>
        ok(data as Body),
      );

      expect(response.isOk()).toBe(true);
      expect(response._unsafeUnwrap()).toEqual(bodyStub);
    });

    it('should fetch data with query params', async () => {
      const bodyStub = { foo: 'bar' };
      type Body = typeof bodyStub;
      server.use(
        http.get(`${url}?key=value`, () =>
          HttpResponse.json(bodyStub, { status: 200 }),
        ),
      );
      const response = await getRequest(ky)<Body>(url, { key: 'value' }, data =>
        ok(data as Body),
      );

      expect(response.isOk()).toBe(true);
      expect(response._unsafeUnwrap()).toEqual(bodyStub);
    });

    it('should be error when fetch failed', async () => {
      const bodyStub = { foo: 'bar' };
      server.use(
        http.get(`${url}`, () => HttpResponse.json(bodyStub, { status: 404 })),
      );
      const response = await getRequest(ky)(url, {}, () => ok({}));

      expect(response.isErr()).toBe(true);
      expect(response._unsafeUnwrapErr()).toEqual(notFound());
    });

    it('should be error when parse failed', async () => {
      const bodyStub = { foo: 'bar' };
      server.use(
        http.get(`${url}`, () => HttpResponse.json(bodyStub, { status: 200 })),
      );
      const response = await getRequest(ky)(url, {}, data => parseError(data));

      expect(response.isErr()).toBe(true);
      expect(response._unsafeUnwrapErr()).toHaveProperty('message', '');
      expect(response._unsafeUnwrapErr()).toHaveProperty('source', bodyStub);
    });
  });
});
