import { describe, expect, afterEach, beforeAll, afterAll } from 'vitest';
import { fc, it } from '@fast-check/vitest';
import { http, HttpResponse } from 'msw';
import { setupServer } from 'msw/node';
import ky from 'ky';
import {
  badRequest,
  conflict,
  forbidden,
  notFound,
  serverError,
  toHttpError,
  unauthorized,
  unknownError,
} from './HttpError';

describe('HttpError', () => {
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
  it('should be BadRequest with body when status code is 400', async () => {
    const bodyStub = { foo: 'bar' };
    server.use(
      http.get(url, () => HttpResponse.json(bodyStub, { status: 400 })),
    );

    try {
      await ky.get(url);
      expect.unreachable('Should be error');
    } catch (e) {
      const result = await toHttpError(e);
      expect(result).toEqual(badRequest(bodyStub));
    }
  });

  it('should be BadRequest with string message when status code is 400', async () => {
    const bodyStub = 'error';
    server.use(
      http.get(url, () => HttpResponse.text(bodyStub, { status: 400 })),
    );

    try {
      await ky.get(url);
      expect.unreachable('Should be error');
    } catch (e) {
      const result = await toHttpError(e);
      expect(result).toEqual(badRequest(bodyStub));
    }
  });

  it('should be Unauthorized when status code is 401', async () => {
    server.use(http.get(url, () => new HttpResponse(null, { status: 401 })));

    try {
      await ky.get(url);
      expect.unreachable('Should be error');
    } catch (e) {
      const result = await toHttpError(e);
      expect(result).toEqual(unauthorized());
    }
  });

  it('should be Forbidden when status code is 403', async () => {
    server.use(http.get(url, () => new HttpResponse(null, { status: 403 })));

    try {
      await ky.get(url);
      expect.unreachable('Should be error');
    } catch (e) {
      const result = await toHttpError(e);
      expect(result).toEqual(forbidden());
    }
  });

  it('should be NotFound when status code is 404', async () => {
    server.use(http.get(url, () => new HttpResponse(null, { status: 404 })));

    try {
      await ky.get(url);
      expect.unreachable('Should be error');
    } catch (e) {
      const result = await toHttpError(e);
      expect(result).toEqual(notFound());
    }
  });

  it('should be Conflict when status code is 409', async () => {
    server.use(http.get(url, () => new HttpResponse(null, { status: 409 })));

    try {
      await ky.get(url);
      expect.unreachable('Should be error');
    } catch (e) {
      const result = await toHttpError(e);
      expect(result).toEqual(conflict());
    }
  });

  it.prop([fc.integer({ min: 500, max: 599 })])(
    'should be ServerError when status code is not 500-599',
    async status => {
      server.use(
        http.get(url, () => new HttpResponse(null, { status: status })),
      );

      try {
        await ky.get(url);
        expect.unreachable('Should be error');
      } catch (e) {
        const result = await toHttpError(e);
        expect(result).toEqual(serverError());
      }
    },
    30000,
  );

  const handledStatusCodes = [400, 401, 403, 404, 409];
  it.prop([
    fc
      .integer({ min: 400, max: 499 })
      .filter(x => !handledStatusCodes.includes(x)),
  ])(
    'should be UnknownError when status code is not 400-499 and not handled',
    async status => {
      server.use(
        http.get(url, () => new HttpResponse(null, { status: status })),
      );

      try {
        await ky.get(url);
        expect.unreachable('Should be error');
      } catch (e) {
        const result = await toHttpError(e);
        expect(result).toEqual(unknownError());
      }
    },
    30000,
  );

  it('should be UnknownError when not HTTPError', async () => {
    const error = new Error('error');
    const result = await toHttpError(error);
    expect(result).toEqual(unknownError());
  });
});
