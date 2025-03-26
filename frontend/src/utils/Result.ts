import * as NeverThrow from 'neverthrow';

export type Result<T, E> = NeverThrow.Result<T, E>;
export type ResultAsync<T, E> = NeverThrow.ResultAsync<T, E>;

export const err = NeverThrow.err;
export const errAsync = NeverThrow.errAsync;

export const ok = NeverThrow.ok;
export const okAsync = NeverThrow.okAsync;

export const fromPromise = NeverThrow.fromPromise;
export const fromThrowable = NeverThrow.fromThrowable;
