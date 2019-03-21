import { createStore, applyMiddleware, compose } from 'redux';
import createSagaMiddleware from 'redux-saga';
import { createLogger } from 'redux-logger';
import rootReducer from '../reducers';
import mySaga from '../sagas';
import DevTools from '../containers/DevTools';

// create the saga middleware
const sagaMiddleware = createSagaMiddleware();

const configureStore = preloadedState => {
  const store = createStore(
    rootReducer,
    preloadedState,
    compose(
      applyMiddleware(sagaMiddleware, createLogger()),
      DevTools.instrument()
    )
  );

  if (module.hot) {
    // Enable Webpack hot module replacement for reducers
    module.hot.accept('../reducers', () => {
      store.replaceReducer(rootReducer);
    });
  }

  sagaMiddleware.run(mySaga);
  return store;
};

export default configureStore;
