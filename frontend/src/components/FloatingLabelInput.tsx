import { Size } from '@/props/Size';
import clsx from 'clsx';
import { JSX, splitProps } from 'solid-js';

export type FloatingLabelInputProps = JSX.IntrinsicElements['input'] &
  Partial<{
    label: string;
  }> &
  Size;

export const FloatingLabelInput = (props: FloatingLabelInputProps) => {
  const [local, inputProps] = splitProps(props, [
    'small',
    'medium',
    'large',
    'class',
    'label',
  ]);
  const inputClass = () =>
    clsx(
      'input',
      {
        'input-sm': local.small,
        'input-md': local.medium,
        'input-lg': local.large,
      },
      local.class,
    );

  const label = () => {
    if (local.label) {
      return local.label;
    }
    if (inputProps.placeholder) {
      return inputProps.placeholder;
    }
    return '';
  };

  return (
    <label class="floating-label">
      <input type="text" class={inputClass()} {...inputProps} />
      <span>{label()}</span>
    </label>
  );
};
