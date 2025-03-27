import { Pagination } from '@/components/Pagination';
import { BrandList } from './BrandList';
import { SearchForm } from './SearchForm';
import { Brand } from '@/schemas/brands/Brand';
import { Sector } from '@/schemas/brands/Sector';
import { Translate } from '@/primitives/createTranslate';
import { BrandSearchCondition } from '@/features/brands/types/BrandSearchCondition';

export type BrandSearchViewProps = {
  sectors: Sector[];
  brands: Brand[];
  page: number;
  maxPage: number;
  selectedSectorId: string;
  code: string;
  brandName: string;
  isLoading: boolean;
  translate: Translate;
  onSearchSubmit: (condition: BrandSearchCondition) => void;
  onConditionChanged: (condition: BrandSearchCondition) => void;
  onPageChanged: (page: number) => void;
  onBrandClick: (brand: Brand) => void;
};

export const BrandSearchView = (props: BrandSearchViewProps) => {
  return (
    <div class="flex flex-col">
      <SearchForm
        disabled={props.isLoading}
        sectors={props.sectors}
        selectedSectorId={props.selectedSectorId}
        code={props.code}
        brandName={props.brandName}
        translate={props.translate}
        onSubmit={props.onSearchSubmit}
        onInputChanged={props.onConditionChanged}
      />
      <BrandList
        brands={props.brands}
        onClick={props.onBrandClick}
        isLoading={props.isLoading}
      />
      <Pagination
        disabled={props.isLoading}
        class="justify-center"
        page={props.page}
        maxPage={props.maxPage}
        onChanged={props.onPageChanged}
      />
    </div>
  );
};
