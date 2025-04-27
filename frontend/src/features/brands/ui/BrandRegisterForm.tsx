import { SelectBox } from '@/components/SelectBox';
import { Translate } from '@/primitives/createTranslate';
import { BrandRegisterFormInput } from '../types/BrandRegisterFormInput';
import { Sector } from '@/schemas/brands/Sector';
import { FloatingLabelInput } from '@/components/FloatingLabelInput';
import {
  getConstraints,
  hasError,
  ValidationError,
} from '@/schemas/common/ValidationError';
import { ValidationHintList } from '@/components/ValidationHintList';

export type BrandRegisterFormProps = {
  input: BrandRegisterFormInput;
  sectors: Sector[];
  validationError: ValidationError | null;
  disabled?: boolean;
  translate: Translate;
  onChange?: (input: BrandRegisterFormInput) => void;
  onSubmit?: (input: BrandRegisterFormInput) => void;
  onCancel?: () => void;
};

export const BrandRegisterForm = (props: BrandRegisterFormProps) => {
  const sectors = () => [
    {
      label: props.translate('brandRegisterFormInputSectorPlaceholder'),
    },
    ...props.sectors.map(sector => ({
      label: sector.name,
      value: sector.id,
    })),
  ];

  const handleChange = (updated: Partial<BrandRegisterFormInput>) => {
    props.onChange?.({
      ...props.input,
      ...updated,
    });
  };

  const hasValidationError = (fieldName: string) =>
    props.validationError ? hasError(fieldName, props.validationError) : false;
  const validationMessages = (fieldName: string) =>
    props.validationError
      ? getConstraints(fieldName, props.validationError)
      : [];

  return (
    <div class="flex flex-col gap-4 items-stretch">
      <div class="w-full">
        <SelectBox
          class="w-64"
          error={hasValidationError('sector')}
          disabled={props.disabled}
          label={props.translate('brandRegisterFormInputSectorLabel')}
          value={props.input.sector?.id}
          items={sectors()}
          onChange={e => {
            const value = e.target.value;
            const selected =
              props.sectors.find(sector => sector.id === value) ?? null;
            handleChange({
              sector: selected,
            });
          }}
        />
        <ValidationHintList
          fieldName="sector"
          constraints={validationMessages('sector')}
          translate={props.translate}
        />
      </div>
      <div class="w-full">
        <FloatingLabelInput
          class="w-24"
          error={hasValidationError('code')}
          disabled={props.disabled}
          minLength={4}
          maxLength={4}
          medium
          label={props.translate('brandRegisterFormInputCodeLabel')}
          placeholder={props.translate('brandRegisterFormInputCodePlaceholder')}
          value={props.input.code}
          onChange={e => {
            const value = e.target.value;
            handleChange({
              code: value,
            });
          }}
        />
        <ValidationHintList
          fieldName="code"
          constraints={validationMessages('code')}
          translate={props.translate}
        />
      </div>
      <div class="w-full">
        <FloatingLabelInput
          error={hasValidationError('name')}
          disabled={props.disabled}
          class="w-full"
          medium
          label={props.translate('brandRegisterFormInputNameLabel')}
          placeholder={props.translate('brandRegisterFormInputNamePlaceholder')}
          value={props.input.name}
          onChange={e => {
            const value = e.target.value;
            handleChange({
              name: value,
            });
          }}
        />
        <ValidationHintList
          fieldName="name"
          constraints={validationMessages('name')}
          translate={props.translate}
        />
      </div>
      <div class="self-center space-x-2">
        <button
          class="btn btn-primary"
          disabled={props.disabled}
          onClick={() => props.onSubmit?.(props.input)}
        >
          {props.translate?.('registerButtonLabel')}
        </button>
        <button
          class="btn btn-secondary"
          disabled={props.disabled}
          onClick={() => props.onCancel?.()}
        >
          {props.translate?.('cancelButtonLabel')}
        </button>
      </div>
    </div>
  );
};
