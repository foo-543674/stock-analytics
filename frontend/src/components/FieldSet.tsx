import clsx from 'clsx';
import { JSX, Show, splitProps } from 'solid-js';

export type FieldSetProps = JSX.IntrinsicElements['fieldset'] &
  Partial<{
    label: string;
    message: string;
  }>;

export const FieldSet = (props: FieldSetProps) => {
  const [local, otherProps] = splitProps(props, [
    'label',
    'children',
    'class',
    'message',
  ]);

  const fieldSetClass = () => clsx('fieldset', local.class);

  return (
    <fieldset class={fieldSetClass()} {...otherProps}>
      <legend class="fieldset-legend">{local.label}</legend>
      {local.children}
      <Show when={local.message}>
        <span class="fieldset-label">{local.message}</span>
      </Show>
    </fieldset>
  );
};
