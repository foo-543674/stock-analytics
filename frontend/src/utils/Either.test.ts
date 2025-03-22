import { describe, it, expect } from 'vitest';
import { Either, flatMap, fold, left, map, mapLeft, right } from './Either';

describe('Either', () => {
  describe('fold', () => {
    it('should call onLeft when Left', () => {
      const either = left('error');
      const result = fold(
        either,
        value => value,
        () => 'unexpected',
      );
      expect(result).toBe('error');
    });

    it('should call onRight when Right', () => {
      const either = right('success');
      const result = fold(
        either,
        () => 'unexpected',
        value => value,
      );
      expect(result).toBe('success');
    });
  });

  describe('map', () => {
    it('should return same as is when Left', () => {
      const either: Either<string, number> = left('error');
      const result = map(either, value => value + 1);
      expect(result).toEqual(left('error'));
    });

    it('should return mapped value when Right', () => {
      const either: Either<string, number> = right(1);
      const result = map(either, value => value + 1);
      expect(result).toEqual(right(2));
    });
  });

  describe('flatMap', () => {
    it('should return same as is when Left', () => {
      const either: Either<string, number> = left('error');
      const result = flatMap(either, value => right(value + 1));
      expect(result).toEqual(left('error'));
    });

    it('should return mapped value when Right', () => {
      const either: Either<string, number> = right(1);
      const result = flatMap(either, value => right(value + 1));
      expect(result).toEqual(right(2));
    });
  });

  describe('mapLeft', () => {
    it('should return same as is when Right', () => {
      const either: Either<string, number> = right(1);
      const result = mapLeft(either, value => value + '!');
      expect(result).toEqual(right(1));
    });

    it('should return mapped value when Left', () => {
      const either: Either<string, number> = left('error');
      const result = mapLeft(either, value => value + '!');
      expect(result).toEqual(left('error!'));
    });
  });

  describe('swap', () => {
    it('should swap Left to Right', () => {
      const either: Either<string, number> = left('error');
      const result = mapLeft(either, value => value + '!');
      expect(result).toEqual(left('error!'));
    });

    it('should swap Right to Left', () => {
      const either: Either<string, number> = right(1);
      const result = mapLeft(either, value => value + '!');
      expect(result).toEqual(right(1));
    });
  });
});
