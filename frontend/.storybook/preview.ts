import { render } from 'solid-js/web';

export const decorators = [
  Story => {
    const solidRoot = document.createElement('div');

    render(Story, solidRoot);

    return solidRoot;
  },
];
