import { Translate } from '@/primitives/createTranslate';
import { ValidationError } from '@/schemas/common/ValidationError';
import { For } from 'solid-js';

type ValidationHintKey = ValidationError['fields'][number]['keys'][number];

export type ValidationHintListProps = Partial<{
  keys: ValidationHintKey[];
  translate: Translate;
}>;

export const ValidationHintList = (props: ValidationHintListProps) => {
  return (
    <div>
      <For each={props.keys}>
        {key => (
          <div role="alert" class="alert alert-error alert-soft py-1 px-4 m-1">
            <span class="text-sm">{props.translate?.(key)}</span>
          </div>
        )}
      </For>
    </div>
  );
};
