import React from 'react';
import { Router, Route, Link, IndexRoute, browserHistory } from 'react-router';
import App from './app.jsx';
import Home from './home.jsx';
import View from './view.jsx';
import SeriesNew from './new.jsx';

function router() {
  return (
      <Router history={browserHistory} >
        <Route path="/" component={App}>
          <IndexRoute component={Home} />
          <Route path="/series/new" component={SeriesNew} />
          <Route path="/series/:id" component={View} />
        </Route>
      </Router>
      );
}

export default router;