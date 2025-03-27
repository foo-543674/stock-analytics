import { JSX } from 'solid-js';

export type LoadingProps = JSX.IntrinsicElements['span'];

export const Loading = (props: LoadingProps) => {
  return (
    <span
      class="loading loading-spinner loading-xl self-center justify-self-center"
      {...props}
    />
  );
};
