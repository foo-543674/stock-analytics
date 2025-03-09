import { createMemo, JSX, splitProps } from 'solid-js';
import styles from './Button.module.css';

export type ButtonProps = JSX.IntrinsicElements['button'] &
  Partial<{
    variant: 'contained' | 'outlined' | 'text';
    color: 'primary' | 'secondary';
  }>;

export const Button = (props: ButtonProps) => {
  const [local, others] = splitProps(props, ['class', 'variant', 'color']);
  const baseClass = createMemo(() => {
    switch (`${local.variant}-${local.color}`) {
      case 'contained-primary':
        return styles.containedPrimary;
      case 'contained-secondary':
        return styles.containedSecondary;
      case 'outlined-primary':
        return styles.outlinedPrimary;
      case 'outlined-secondary':
        return styles.outlinedSecondary;
      case 'text-primary':
        return styles.textPrimary;
      case 'text-secondary':
        return styles.textSecondary;
      default:
        return styles.containedPrimary;
    }
  });

  return <button class={`${baseClass()} ${local.class}`} {...others} />;
};
