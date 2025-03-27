import { ApiClient } from '@/data-access/ApiClient';
import { Translate } from '@/primitives/createTranslate';
import { createBrandsList } from '../primitives/createBrandsList';
import { BrandSearchView } from '../ui/BrandSearchView';

export type BrandSearchContainerProps = {
  initialApiClient: ApiClient;
  translate: Translate;
};

export const BrandSearchContainer = (props: BrandSearchContainerProps) => {
  const brands = createBrandsList(props.initialApiClient);

  return (
    <BrandSearchView
      brands={brands().brands}
      page={brands().page}
      maxPage={brands().maxPage}
      sectors={[]}
      selectedSectorId={brands().filter.sectorId ?? ''}
      code={brands().filter.code ?? ''}
      brandName={brands().filter.brandName ?? ''}
      isLoading={brands().isLoading}
      translate={props.translate}
      onPageChanged={page => brands().setPage(page)}
      onConditionChanged={condition => brands().setFilter(condition)}
      onSearchSubmit={() => brands().refetch()}
      onBrandClick={() => {}}
    />
  );
};
