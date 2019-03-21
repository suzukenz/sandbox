import { createStore, applyMiddleware } from 'redux';
import createSagaMiddleware from 'redux-saga';
import rootReducer from '../reducers';
import mySaga from '../sagas';

// create the saga middleware
const sagaMiddleware = createSagaMiddleware();

const configureStore = preloadedState => {
  const store = createStore(
    rootReducer,
    preloadedState,
    applyMiddleware(sagaMiddleware)
  );

  sagaMiddleware.run(mySaga);
  return store;
};

export default configureStore;
