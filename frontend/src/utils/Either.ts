export type Left<T> = {
  _tag: 'Left';
  value: T;
};
export type Right<T> = {
  _tag: 'Right';
  value: T;
};
export type Either<L, R> = Left<L> | Right<R>;
export const left = <L, R>(value: L): Either<L, R> => ({ _tag: 'Left', value });
export const right = <L, R>(value: R): Either<L, R> => ({
  _tag: 'Right',
  value,
});

export const fold = <L, R, T>(
  either: Either<L, R>,
  onLeft: (value: L) => T,
  onRight: (value: R) => T,
) => {
  switch (either._tag) {
    case 'Left':
      return onLeft(either.value);
    case 'Right':
      return onRight(either.value);
  }
};

export const map = <L, R, T>(
  either: Either<L, R>,
  f: (value: R) => T,
): Either<L, T> => {
  switch (either._tag) {
    case 'Left':
      return either;
    case 'Right':
      return right(f(either.value));
  }
};

export const flatMap = <L, R, T>(
  either: Either<L, R>,
  f: (value: R) => Either<L, T>,
): Either<L, T> => {
  switch (either._tag) {
    case 'Left':
      return either;
    case 'Right':
      return f(either.value);
  }
};

export const mapLeft = <L, R, T>(
  either: Either<L, R>,
  f: (value: L) => T,
): Either<T, R> => {
  switch (either._tag) {
    case 'Left':
      return left(f(either.value));
    case 'Right':
      return either;
  }
};

export const swap = <L, R>(either: Either<L, R>): Either<R, L> => {
  switch (either._tag) {
    case 'Left':
      return right(either.value);
    case 'Right':
      return left(either.value);
  }
};
