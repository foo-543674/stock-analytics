import { createWindowSize } from '@/primitives/createWindowSize';
import { generatePages, PaginationItem } from '@/utils/PaginationHelper';
import clsx from 'clsx';
import { For, Show } from 'solid-js';

export type PaginationProps = { page: number; maxPage: number } & Partial<{
  class: string;
  onChanged: (page: number) => void;
}>;

const desktopVisibleRange = 5;
const tabletVisibleRange = 3;
const mobileVisibleRange = 1;

export const Pagination = (props: PaginationProps) => {
  const rootClass = () => clsx('join', props.class);
  const baseClass = () => clsx('join-item', 'btn', 'btn-ghost');
  const pageNumberClass = (num: PaginationItem) =>
    clsx(baseClass(), {
      'btn-active': props.page === num,
      'btn-disabled': props.page === num || num === '...',
      'p-0': num === '...',
    });

  const arrotButtonClass = (direction: 'left' | 'right') =>
    clsx(baseClass(), {
      'btn-disabled':
        (direction === 'left' && props.page === 1) ||
        (direction === 'right' && props.page === props.maxPage),
    });

  const [, deviceType] = createWindowSize();
  const pagesNumbers = () =>
    generatePages(
      props.page ?? 1,
      props.maxPage ?? 1,
      deviceType() === 'desktop'
        ? desktopVisibleRange
        : deviceType() === 'tablet'
          ? tabletVisibleRange
          : mobileVisibleRange,
    );

  const handleClick = (num: PaginationItem) => {
    if (num === '...') {
      return;
    }
    props.onChanged?.(Math.max(1, Math.min(num, props.maxPage)));
  };

  return (
    <div class={rootClass()}>
      <button
        data-testid="to-first-button"
        class={arrotButtonClass('left')}
        onClick={() => handleClick(1)}
      >
        {'<<'}
      </button>
      <Show when={deviceType() !== 'mobile'}>
        <button
          data-testid="to-prev-button"
          class={arrotButtonClass('left')}
          onClick={() => handleClick(props.page - 1)}
        >
          {'<'}
        </button>
      </Show>
      <For each={pagesNumbers()}>
        {num => (
          <button
            data-testid={`${num === '...' ? 'ommitted-button' : `to-${num}-button`}`}
            class={pageNumberClass(num)}
            onClick={() => handleClick(num)}
          >
            {num}
          </button>
        )}
      </For>
      <Show when={deviceType() !== 'mobile'}>
        <button
          data-testid="to-next-button"
          class={arrotButtonClass('right')}
          onClick={() => handleClick(props.page + 1)}
        >
          {'>'}
        </button>
      </Show>
      <button
        data-testid="to-last-button"
        class={arrotButtonClass('right')}
        onClick={() => handleClick(props.maxPage)}
      >
        {'>>'}
      </button>
    </div>
  );
};
