import humps from 'humps';

export const isObject = (value: unknown): value is Record<string, unknown> => {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    return false;
  }

  const proto = Object.getPrototypeOf(value);
  return proto === Object.prototype || proto === null;
};

export const camelizeKeys = (
  source: Record<string, unknown>,
): Record<string, unknown> => {
  return humps.camelizeKeys(source) as Record<string, unknown>;
};

export const filterProperties = <T extends object>(
  obj: T,
  filterFn: (key: string, value: unknown) => boolean,
): Partial<T> => {
  return Object.fromEntries(
    Object.entries(obj).filter(([key, value]) => filterFn(key, value)),
  ) as Partial<T>;
};

export const filterUndefinedProperties = <T extends object>(
  obj: T,
): Partial<T> => filterProperties(obj, (_key, value) => value !== undefined);
