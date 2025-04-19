import clsx from 'clsx';
import { For, JSX, splitProps } from 'solid-js';

export type SelectBoxItem = {
  label: string;
  value?: string;
};
export type SelectBoxProps = Partial<{
  label: string;
  items: SelectBoxItem[];
  error?: boolean;
}> &
  JSX.IntrinsicElements['select'];

export const SelectBox = (props: SelectBoxProps) => {
  const [local, selectProps] = splitProps(props, [
    'label',
    'items',
    'class',
    'error',
    'value',
  ]);

  const isSelected = (item: SelectBoxItem) => {
    return local.value === item.value;
  };

  const rootClass = () =>
    clsx('select', {
      'select-error': local.error,
    });

  return (
    <label class={rootClass()}>
      <span class="label">{local.label}</span>
      <select class={local.class} value={local.value} {...selectProps}>
        <For each={local.items}>
          {item => (
            <option value={item.value} selected={isSelected(item)}>
              {item.label}
            </option>
          )}
        </For>
      </select>
    </label>
  );
};
