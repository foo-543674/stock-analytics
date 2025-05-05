import stringFormat from 'string-format';

export const format = (
  source: string,
  namedArgs?: Record<string, string>,
  ...args: string[]
): string => {
  if (!namedArgs) {
    return stringFormat(source, ...args);
  }

  const regex = /\{([0-9]+)\}/g;
  const matches = source.match(regex);

  if (!matches) {
    return stringFormat(source, namedArgs);
  }

  const placeholderPrefix = `__tmp_placeholder__`;
  const { source: indexReplacedSource, placeholders } = matches.reduce(
    (acc, match, index) => {
      const placeholder = `${placeholderPrefix}${index}`;
      return {
        source: acc.source.replace(match, `{${placeholder}}`),
        placeholders: [...acc.placeholders, placeholder],
      };
    },
    { source, placeholders: [] as string[] },
  );

  const mergedArgs = Object.assign(
    {},
    namedArgs,
    ...args.map((arg, index) => ({ [placeholders[index]]: arg })),
  );
  return stringFormat(indexReplacedSource, mergedArgs);
};
