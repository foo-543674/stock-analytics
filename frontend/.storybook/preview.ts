import { render } from 'solid-js/web';
import '../src/index.css';

export const decorators = [
  Story => {
    const solidRoot = document.createElement('div');
    solidRoot.setAttribute('data-theme', 'light');

    render(Story, solidRoot);

    return solidRoot;
  },
];
