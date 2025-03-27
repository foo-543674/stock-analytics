import { BrandSearchContainer } from '@/features/brands/containers/BrandSearchContainer';
import { PageBase } from '../PageBase';

export const BrandSearchPage = () => {
  //TODO: handle query params

  return (
    <PageBase>
      {({ initialApiClient, translate }) => (
        <BrandSearchContainer
          initialApiClient={initialApiClient}
          translate={translate}
        />
      )}
    </PageBase>
  );
};
