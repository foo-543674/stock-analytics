import { FieldSet } from '@/components/FieldSet';
import { BrandSearchCondition } from '@/features/brands/BrandSearchCondition';
import { Translate } from '@/primitives/createTranslate';
import { Sector } from '@/schemas/Sector';
import { FiSearch } from 'solid-icons/fi';
import { For, Show } from 'solid-js';

export type SearchFormProps = Partial<{
  sectors: Sector[];
  selectedSectorId: string;
  code: string;
  brandName: string;
  disabled?: boolean;
  translate: Translate;
  onSubmit: (condition: BrandSearchCondition) => void;
  onInputChanged: (condition: BrandSearchCondition) => void;
}>;

export const SearchForm = (props: SearchFormProps) => {
  const selectedSectorId = () => props.selectedSectorId ?? '';
  const code = () => props.code ?? '';
  const brandName = () => props.brandName ?? '';
  return (
    <>
      <div class="flex flex-col lg:flex-row space-x-4">
        <FieldSet class="w-50" label={props.translate?.('selectSectorLabel')}>
          <select
            class="select"
            disabled={props.disabled}
            value={selectedSectorId()}
            onChange={e =>
              props.onInputChanged?.({
                sectorId: e.currentTarget.value,
                code: code(),
                brandName: brandName(),
              })
            }
          >
            <Show when={props.sectors?.length ?? 0 > 0}>
              <option value="">{props.translate?.('optionAll')}</option>
            </Show>
            <For
              each={props.sectors}
              fallback={
                <option disabled>{props.translate?.('loading')}</option>
              }
            >
              {sector => <option value={sector.id}>{sector.name}</option>}
            </For>
          </select>
        </FieldSet>
        <FieldSet class="w-30" label={props.translate?.('inputCodeLabel')}>
          <label class="input">
            <FiSearch class="h-[1em] opacity-50" />
            <input
              type="search"
              placeholder="0050"
              disabled={props.disabled}
              value={code()}
              onInput={e =>
                props.onInputChanged?.({
                  sectorId: selectedSectorId(),
                  code: e.currentTarget.value,
                  brandName: brandName(),
                })
              }
            />
          </label>
        </FieldSet>
        <FieldSet
          class="flex-1"
          label={props.translate?.('inputBrandNameLabel')}
        >
          <label class="input w-full">
            <FiSearch class="h-[1em] opacity-50" />
            <input
              type="search"
              placeholder="Search"
              disabled={props.disabled}
              value={brandName()}
              onInput={e =>
                props.onInputChanged?.({
                  sectorId: selectedSectorId(),
                  code: code(),
                  brandName: e.currentTarget.value,
                })
              }
            />
          </label>
        </FieldSet>
      </div>
      <div class="flex justify-center mt-4">
        <button
          class="btn btn-primary"
          disabled={props.disabled}
          onClick={() =>
            props.onSubmit?.({
              sectorId: selectedSectorId(),
              code: code(),
              brandName: brandName(),
            })
          }
        >
          {props.translate?.('searchButtonLabel')}
        </button>
      </div>
    </>
  );
};
