import { Pagination } from '@/components/Pagination';
import { BrandList } from './BrandList';
import { SearchForm } from './SearchForm';
import { Brand } from '@/schemas/Brand';
import { Sector } from '@/schemas/Sector';
import { Translate } from '@/primitives/createTranslate';
import { BrandSearchCondition } from '@/features/brands/BrandSearchCondition';

export type BrandSearchViewProps = {
  sectors: Sector[];
  brands: Brand[];
  page: number;
  maxPage: number;
  selectedSectorId: string;
  code: string;
  brandName: string;
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
        sectors={props.sectors}
        selectedSectorId={props.selectedSectorId}
        code={props.code}
        brandName={props.brandName}
        translate={props.translate}
        onSubmit={props.onSearchSubmit}
        onInputChanged={props.onConditionChanged}
      />
      <BrandList brands={props.brands} onClick={props.onBrandClick} />
      <Pagination
        class="justify-center"
        page={props.page}
        maxPage={props.maxPage}
        onChanged={props.onPageChanged}
      />
    </div>
  );
};
