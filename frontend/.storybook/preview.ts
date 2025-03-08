import { render } from 'solid-js/web';
import '../src/index.css';

export const decorators = [
  Story => {
    const solidRoot = document.createElement('div');

    render(Story, solidRoot);

    return solidRoot;
  },
];
