import { SelectBox } from '@/components/SelectBox';
import { Translate } from '@/primitives/createTranslate';
import { BrandRegisterFormInput } from '../types/BrandRegisterFormInput';
import { Sector } from '@/schemas/brands/Sector';
import { FloatingLabelInput } from '@/components/FloatingLabelInput';
import { ValidationError } from '@/schemas/common/ValidationError';
import { ValidationHintList } from '@/components/ValidationHintList';

export type BrandRegisterFormProps = {
  input: BrandRegisterFormInput;
  sectors: Sector[];
  validationError: ValidationError | null;
  disabled?: boolean;
  translate: Translate;
  onChange: (input: BrandRegisterFormInput) => void;
  onSubmit?: (input: BrandRegisterFormInput) => void;
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
    props.onChange({
      ...props.input,
      ...updated,
    });
  };

  const hasError = (fieldName: string) => {
    return (
      props.validationError?.fields.some(f => f.name === fieldName) ?? false
    );
  };
  const validationMessages = (fieldName: string) => {
    const field = props.validationError?.fields.find(f => f.name === fieldName);
    return field?.keys ?? [];
  };

  return (
    <div class="flex flex-col gap-4 items-stretch">
      <div class="w-full">
        <SelectBox
          class="w-64"
          error={hasError('sector')}
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
          keys={validationMessages('sector')}
          translate={props.translate}
        />
      </div>
      <div class="w-full">
        <FloatingLabelInput
          class="w-24"
          error={hasError('code')}
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
          keys={validationMessages('code')}
          translate={props.translate}
        />
      </div>
      <div class="w-full">
        <FloatingLabelInput
          error={hasError('name')}
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
          keys={validationMessages('name')}
          translate={props.translate}
        />
      </div>
      <div class="self-center">
        <button
          class="btn btn-primary"
          disabled={props.disabled}
          onClick={() => props.onSubmit?.(props.input)}
        >
          {props.translate?.('registerButtonLabel')}
        </button>
      </div>
    </div>
  );
};
