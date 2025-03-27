import { BrandSearchPage } from '@/pages/brands/BrandsSearchPage';
import { Home } from '@/pages/Home';
import { Route, Router } from '@solidjs/router';

export const PageRouter = () => {
  return (
    <Router>
      <Route path="/" component={Home} />
      <Route path="/brands" component={BrandSearchPage} />
    </Router>
  );
};
