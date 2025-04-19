import { For, JSX, splitProps } from 'solid-js';

export type SelectBoxItem = {
  label: string;
  value: string;
};
export type SelectBoxProps = Partial<{
  label: string;
  items: SelectBoxItem[];
  value: string;
}> &
  JSX.IntrinsicElements['select'];

export const SelectBox = (props: SelectBoxProps) => {
  const [local, selectProps] = splitProps(props, [
    'label',
    'items',
    'class',
    'value',
  ]);
  return (
    <label class="select">
      <span class="label">{local.label}</span>
      <select class={local.class} value={local.value} {...selectProps}>
        <For each={local.items}>
          {item => <option value={item.value}>{item.label}</option>}
        </For>
      </select>
    </label>
  );
};
