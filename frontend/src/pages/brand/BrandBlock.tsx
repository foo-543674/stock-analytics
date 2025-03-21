import { Brand } from '@/schemas/Brand';
import { isTextSelected } from '@/utils/BrowserHelper';
import clsx from 'clsx';

export type BrandBlockProps = {
  brand: Brand;
} & Partial<{
  class: string;
  onClick: (brand: Brand) => void;
}>;

export const BrandBlock = (props: BrandBlockProps) => {
  const handleClick = () => {
    if (isTextSelected()) {
      return;
    }

    props.onClick?.(props.brand);
  };

  const rootClass = () =>
    clsx(
      'flex',
      'flex-col',
      'bg-base-100',
      'border-base-300',
      'border',
      'hover:border-2',
      'cursor-pointer',
      'select-text',
      props.class,
    );
  return (
    <div
      data-testid={`brand-block-${props.brand.id}`}
      class={rootClass()}
      onClick={handleClick}
    >
      <div class="flex-none px-4 pt-4 text-sm">
        <span class="font-bold">{props.brand.code}</span>
        <span class="font-bold mx-2">{props.brand.sector.group}</span>
        <span class="font-bold mx-2">{props.brand.sector.name}</span>
      </div>
      <div class="flex-auto p-4 text-lg">
        <span>{props.brand.name}</span>
      </div>
    </div>
  );
};
