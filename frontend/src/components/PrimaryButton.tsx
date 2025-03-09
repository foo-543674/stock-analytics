import { JSX, splitProps } from 'solid-js';
import styles from './PrimaryButton.module.css';

export type PrimaryButtonProps = JSX.IntrinsicElements['button'];

export const PrimaryButton = (props: PrimaryButtonProps) => {
  const [local, others] = splitProps(props, ['class']);

  return (
    <button class={`${styles.primaryButton} ${local.class}`} {...others} />
  );
};
