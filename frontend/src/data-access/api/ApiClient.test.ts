import { describe, expect, afterEach } from 'vitest';
import { it, fc } from '@fast-check/vitest';
import axios from 'axios';
import MockAdapter from 'axios-mock-adapter';
import { createApiClient } from './ApiClient';
import { left, right } from '@/utils/Either';
import {
  badRequest,
  conflict,
  forbidden,
  notFound,
  serverError,
  unauthorized,
  unknownError,
  validationFailed,
} from './ApiError';
import {
  BAD_REQUEST,
  CONFLICT,
  FORBIDDEN,
  NOT_FOUND,
  UNAUTHORIZED,
} from '@/utils/HttpHelper';

describe('ApiClient', () => {
  const mock = new MockAdapter(axios);
  afterEach(() => {
    mock.reset();
  });

  describe('get', () => {
    it('should return right when successed', async () => {
      mock
        .onGet('https://api.example.com/users')
        .reply(200, { id: 1, name: 'Test' });

      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.get('/users', {}, data => data);

      expect(result).toEqual(right({ id: 1, name: 'Test' }));
    });

    it('should return left when failed', async () => {
      mock.onGet('https://api.example.com/users').reply(500);

      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.get('/users', {}, data => data);

      expect(result).toEqual(left(serverError()));
    });
  });

  describe('post', () => {
    it('should return right when successed', async () => {
      mock
        .onPost('https://api.example.com/users')
        .reply(200, { id: 1, name: 'Test' });

      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.post('/users', {}, data => data);

      expect(result).toEqual(right({ id: 1, name: 'Test' }));
    });

    it('should return left when failed', async () => {
      mock.onPost('https://api.example.com/users').reply(500);

      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.post('/users', {}, data => data);

      expect(result).toEqual(left(serverError()));
    });
  });

  describe('put', () => {
    it('should return right when successed', async () => {
      mock
        .onPut('https://api.example.com/users')
        .reply(200, { id: 1, name: 'Test' });

      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.put('/users', {}, data => data);

      expect(result).toEqual(right({ id: 1, name: 'Test' }));
    });

    it('should return left when failed', async () => {
      mock.onPut('https://api.example.com/users').reply(500);

      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.put('/users', {}, data => data);

      expect(result).toEqual(left(serverError()));
    });
  });

  describe('delete', () => {
    it('should return right when successed', async () => {
      mock
        .onDelete('https://api.example.com/users')
        .reply(200, { id: 1, name: 'Test' });

      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.delete('/users', data => data);

      expect(result).toEqual(right({ id: 1, name: 'Test' }));
    });

    it('should return left when failed', async () => {
      mock.onDelete('https://api.example.com/users').reply(500);

      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.delete('/users', data => data);

      expect(result).toEqual(left(serverError()));
    });
  });

  describe('handleError', () => {
    it('should return validationError when status is 400 and body is validation error', async () => {
      const validationError = {
        fields: [
          {
            name: 'name',
            keys: ['validation.required' as const],
          },
        ],
      };
      mock.onGet('https://api.example.com/users').reply(400, validationError);
      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.get('/users', {}, data => data);

      expect(result).toEqual(left(validationFailed(validationError)));
    });

    it('should return BadRequest when status is 400 and body is not validation error', async () => {
      mock
        .onGet('https://api.example.com/users')
        .reply(400, { message: 'invalid' });
      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.get('/users', {}, data => data);

      expect(result).toEqual(left(badRequest({ message: 'invalid' })));
    });

    it('should return NotFound when status is 404', async () => {
      mock.onGet('https://api.example.com/users').reply(NOT_FOUND);
      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.get('/users', {}, data => data);

      expect(result).toEqual(left(notFound()));
    });

    it('should return Conflict when status is 409', async () => {
      mock.onGet('https://api.example.com/users').reply(CONFLICT);
      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.get('/users', {}, data => data);

      expect(result).toEqual(left(conflict()));
    });

    it('should return Forbidden when status is 403', async () => {
      mock.onGet('https://api.example.com/users').reply(FORBIDDEN);
      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.get('/users', {}, data => data);

      expect(result).toEqual(left(forbidden()));
    });

    it('should return Unauthorized when status is 401', async () => {
      mock.onGet('https://api.example.com/users').reply(UNAUTHORIZED);
      const apiClient = createApiClient('https://api.example.com');
      const result = await apiClient.get('/users', {}, data => data);

      expect(result).toEqual(left(unauthorized()));
    });

    it.prop([fc.integer({ min: 500, max: 599 })])(
      'should return serverError when status is 500',
      async status => {
        mock.onGet('https://api.example.com/users').reply(status);
        const apiClient = createApiClient('https://api.example.com');
        const result = await apiClient.get('/users', {}, data => data);

        expect(result).toEqual(left(serverError()));
      },
    );

    const knownClientErrors = [
      BAD_REQUEST,
      UNAUTHORIZED,
      FORBIDDEN,
      NOT_FOUND,
      CONFLICT,
    ];
    it.prop([
      fc
        .integer({ min: 401, max: 499 })
        .filter(x => !knownClientErrors.includes(x)),
    ])(
      'should return unknownError when status is not defined',
      async status => {
        mock.onGet('https://api.example.com/users').reply(status);
        const apiClient = createApiClient('https://api.example.com');
        const result = await apiClient.get('/users', {}, data => data);

        expect(result).toEqual(left(unknownError()));
      },
    );
  });
});
