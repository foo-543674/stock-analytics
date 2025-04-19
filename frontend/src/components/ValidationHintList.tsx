import { Translate } from '@/primitives/createTranslate';
import { ValidationError } from '@/schemas/common/ValidationError';
import { For } from 'solid-js';

type ValidationHintKey = ValidationError['fields'][number]['keys'][number];

export type ValidationHintListProps = Partial<{
  keys: ValidationHintKey[];
  translate: Translate;
}>;

const translateKeys: Record<ValidationHintKey, string> = {
  'validation.required': 'validationRequired',
  'validation.duplicate': 'validationDuplicate',
  'validation.max_length': 'validationMaxLength',
  'validation.length_equals': 'validationLengthEquals',
  'validation.numeric_only': 'validationNumericOnly',
  'validation.resource_not_found': 'validationResourceNotFound',
  'validation.ulid': 'validationUlid',
};

export const ValidationHintList = (props: ValidationHintListProps) => {
  return (
    <div>
      <For each={props.keys}>
        {key => (
          <div role="alert" class="alert alert-error alert-soft py-1 px-4 m-1">
            <span class="text-sm">{props.translate?.(translateKeys[key])}</span>
          </div>
        )}
      </For>
    </div>
  );
};
