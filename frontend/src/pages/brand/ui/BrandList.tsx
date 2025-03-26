import { Brand } from '@/schemas/brands/Brand';
import { For, Show } from 'solid-js';
import { BrandBlock } from './BrandBlock';
import clsx from 'clsx';

export type BrandListProps = Partial<{
  class: string;
  brands: Brand[];
  isLoading: boolean;
  onClick: (brand: Brand) => void;
}>;

export const BrandList = (props: BrandListProps) => {
  const rootClass = () =>
    clsx(
      'flex',
      'flex-col',
      'min-h-[50vh]',
      'items-streach',
      'justify-center',
      props.class,
    );

  return (
    <div class={rootClass()}>
      <Show
        when={!props.isLoading}
        fallback={
          <span class="loading loading-spinner loading-xl self-center justify-self-center" />
        }
      >
        <ul class="list bg-base-100 shaddow-md">
          <For each={props.brands}>
            {brand => (
              <li class="p-2">
                <BrandBlock brand={brand} onClick={props.onClick} />
              </li>
            )}
          </For>
        </ul>
      </Show>
    </div>
  );
};
