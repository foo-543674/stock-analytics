import { Either, left, right } from '@/utils/Either';
import {
  ApiError,
  badRequest,
  conflict,
  forbidden,
  notFound,
  serverError,
  unauthorized,
  unknownError,
  validationFailed,
} from './ApiError';
import axios, { AxiosInstance } from 'axios';
import {
  BAD_REQUEST,
  CONFLICT,
  FORBIDDEN,
  isServerError,
  NOT_FOUND,
  UNAUTHORIZED,
} from '@/utils/HttpHelper';
import { isValidationError } from '@/schemas/ValidationError';

export type ApiResponse<T> = Either<ApiError, T>;

export interface ApiClient {
  get: <T>(
    path: string,
    query: Record<string, string>,
    parseResponse: (source: unknown) => T,
  ) => Promise<ApiResponse<T>>;
  post: <T>(
    path: string,
    body: T,
    parseResponse: (source: unknown) => T,
  ) => Promise<ApiResponse<T>>;
  put: <T>(
    path: string,
    body: T,
    parseResponse: (source: unknown) => T,
  ) => Promise<ApiResponse<T>>;
  delete: <T>(
    path: string,
    parseResponse: (source: unknown) => T,
  ) => Promise<ApiResponse<T>>;
}

const handleError = (error: unknown) => {
  if (axios.isAxiosError(error)) {
    if (!error.response) {
      return unknownError();
    }
    switch (error.response.status) {
      case BAD_REQUEST: {
        if (isValidationError(error.response.data)) {
          return validationFailed(error.response.data);
        }
        return badRequest(error.response.data);
      }
      case NOT_FOUND:
        return notFound();
      case CONFLICT:
        return conflict();
      case FORBIDDEN:
        return forbidden();
      case UNAUTHORIZED:
        return unauthorized();
      default: {
        if (isServerError(error.response.status)) {
          return serverError();
        } else {
          return unknownError();
        }
      }
    }
  }
  return unknownError();
};

const sendGetRequest = async <T>(
  base: AxiosInstance,
  path: string,
  query: Record<string, string>,
  parseResponse: (source: unknown) => T,
): Promise<ApiResponse<T>> => {
  try {
    const response = await base.get(path, { params: query });
    return right(parseResponse(response.data));
  } catch (error) {
    return left(handleError(error));
  }
};

const sendPostRequest = async <T, D>(
  base: AxiosInstance,
  path: string,
  body: D,
  parseResponse: (source: unknown) => T,
): Promise<ApiResponse<T>> => {
  try {
    const response = await base.post(path, body);
    return right(parseResponse(response.data));
  } catch (error) {
    return left(handleError(error));
  }
};

const sendPutRequest = async <T, D>(
  base: AxiosInstance,
  path: string,
  body: D,
  parseResponse: (source: unknown) => T,
): Promise<ApiResponse<T>> => {
  try {
    const response = await base.put(path, body);
    return right(parseResponse(response.data));
  } catch (error) {
    return left(handleError(error));
  }
};

const sendDeleteRequest = async <T>(
  base: AxiosInstance,
  path: string,
  parseResponse: (source: unknown) => T,
): Promise<ApiResponse<T>> => {
  try {
    const response = await base.delete(path);
    return right(parseResponse(response.data));
  } catch (error) {
    return left(handleError(error));
  }
};

export const createApiClient = (baseUrl: string): ApiClient => {
  const base = axios.create({
    baseURL: baseUrl,
    headers: {},
  });

  return {
    get: <T>(
      path: string,
      query: Record<string, string>,
      parseResponse: (source: unknown) => T,
    ) => sendGetRequest<T>(base, path, query, parseResponse),
    post: <T>(path: string, body: T, parseResponse: (source: unknown) => T) =>
      sendPostRequest(base, path, body, parseResponse),
    put: <T>(path: string, body: T, parseResponse: (source: unknown) => T) =>
      sendPutRequest(base, path, body, parseResponse),
    delete: <T>(path: string, parseResponse: (source: unknown) => T) =>
      sendDeleteRequest(base, path, parseResponse),
  };
};
