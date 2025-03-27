import { BrandSearchContainer } from '@/features/brands/containers/BrandSearchContainer';
import { PageBase } from '../PageBase';

export const BrandSearchPage = () => {
  //TODO: handle query params

  return (
    <PageBase>
      {({ apiClient, translate }) => (
        <BrandSearchContainer apiClient={apiClient()} translate={translate()} />
      )}
    </PageBase>
  );
};
