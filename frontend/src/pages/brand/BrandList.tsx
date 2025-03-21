import { Brand } from '@/schemas/Brand';
import { For } from 'solid-js';
import { BrandBlock } from './BrandBlock';

export type BrandListProps = Partial<{
  brands: Brand[];
  onClick: (brand: Brand) => void;
}>;

export const BrandList = (props: BrandListProps) => {
  return (
    <ul class="list bg-base-100 shaddow-md">
      <For each={props.brands}>
        {brand => (
          <li class="p-2">
            <BrandBlock brand={brand} onClick={props.onClick} />
          </li>
        )}
      </For>
    </ul>
  );
};
