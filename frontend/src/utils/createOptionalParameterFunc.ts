export const createOptionalParameterFunc = <T, R>(
  defaultValue: T,
  f: (p: T) => R,
): ((p: Partial<T>) => R) => {
  return (value: Partial<T>) => f({ ...defaultValue, ...value });
};
