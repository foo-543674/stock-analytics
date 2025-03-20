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
