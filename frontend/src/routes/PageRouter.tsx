import { Home } from '@/pages/Home';
import { Route, Router } from '@solidjs/router';

export const PageRouter = () => {
  return (
    <Router>
      <Route path="/" component={Home} />
    </Router>
  );
};
