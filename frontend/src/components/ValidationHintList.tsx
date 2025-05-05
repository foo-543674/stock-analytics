import { Translate } from '@/primitives/createTranslate';
import {
  ValidationConstraint,
  ValidationRule,
} from '@/schemas/common/ValidationError';
import { format } from '@/utils/StringHelper';
import { For } from 'solid-js';

export type ValidationHintListProps = Partial<{
  fieldName: string;
  constraints: ValidationConstraint[];
  translate: Translate;
}>;

const translateKeys: Record<ValidationRule, string> = {
  'validation.required': 'validationRequired',
  'validation.duplicate': 'validationDuplicate',
  'validation.max_length': 'validationMaxLength',
  'validation.length_equals': 'validationLengthEquals',
  'validation.numeric_only': 'validationNumericOnly',
  'validation.resource_not_found': 'validationResourceNotFound',
  'validation.ulid': 'validationUlid',
};

export const ValidationHintList = (props: ValidationHintListProps) => {
  const formatMessage = (constraint: ValidationConstraint) => {
    const message = props.translate?.(translateKeys[constraint.rule]) ?? '';
    return format(
      message,
      {
        field: props.fieldName ?? 'field',
      },
      ...(constraint.args ?? []),
    );
  };

  return (
    <div>
      <For each={props.constraints}>
        {constraint => (
          <div role="alert" class="alert alert-error alert-soft py-1 px-4 m-1">
            <span class="text-sm">{formatMessage(constraint)}</span>
          </div>
        )}
      </For>
    </div>
  );
};
