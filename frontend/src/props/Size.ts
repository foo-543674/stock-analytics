type Small = {
  small: true;
  medium?: never;
  large?: never;
};

type Medium = {
  small?: never;
  medium: true;
  large?: never;
};

type Large = {
  small?: never;
  medium?: never;
  large: true;
};

export type Size = Small | Medium | Large;
