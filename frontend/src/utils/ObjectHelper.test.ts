import { describe, it, expect } from 'vitest';
import { camelizeKeys, isObject } from './ObjectHelper';

describe('ObjectHelper', () => {
  describe('isObject', () => {
    it('should be true when value is object', () => {
      expect(isObject({})).toBe(true);
    });

    it('should be false when value is null', () => {
      expect(isObject(null)).toBe(false);
    });

    it('should be false when value is undefined', () => {
      expect(isObject(undefined)).toBe(false);
    });

    it('should be false when value is string', () => {
      expect(isObject('string')).toBe(false);
    });

    it('should be false when value is number', () => {
      expect(isObject(1)).toBe(false);
    });

    it('should be false when value is boolean', () => {
      expect(isObject(true)).toBe(false);
    });

    it('should be false when value is array', () => {
      expect(isObject([])).toBe(false);
    });

    it('should be false when value is function', () => {
      expect(isObject(() => {})).toBe(false);
    });

    it('should be false when value is symbol', () => {
      expect(isObject(Symbol('symbol'))).toBe(false);
    });

    it('should be false when value is Promise', () => {
      expect(isObject(Promise.resolve())).toBe(false);
    });

    it('should be false when value is Date', () => {
      expect(isObject(new Date())).toBe(false);
    });

    it('should be false when value is RegExp', () => {
      expect(isObject(/regexp/)).toBe(false);
    });

    it('should be false when value is Error', () => {
      expect(isObject(new Error())).toBe(false);
    });
  });

  describe('camelizeKeys', () => {
    it('should convert snake_case to camelCase', () => {
      const source = {
        id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
        name: 'some name',
        code: '0000',
        test_key: 'test value',
        test_key_foo: 'test value 2',
      };
      const result = camelizeKeys(source);
      expect(result).toEqual({
        id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
        name: 'some name',
        code: '0000',
        testKey: 'test value',
        testKeyFoo: 'test value 2',
      });
    });

    it('should convert snake_case to camelCase for nested object', () => {
      const source = {
        id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
        name: 'some name',
        code: '0000',
        nested: {
          test_key: 'test value',
          test_key_foo: 'test value 2',
        },
      };
      const result = camelizeKeys(source);
      expect(result).toEqual({
        id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
        name: 'some name',
        code: '0000',
        nested: {
          testKey: 'test value',
          testKeyFoo: 'test value 2',
        },
      });
    });

    it("should keep as is when value isn't snake_case", () => {
      const source = {
        id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
        name: 'some name',
        code: '0000',
        testKey: 'test value',
        testKeyFoo: 'test value 2',
      };
      const result = camelizeKeys(source);
      expect(result).toEqual({
        id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
        name: 'some name',
        code: '0000',
        testKey: 'test value',
        testKeyFoo: 'test value 2',
      });
    });
  });
});
