import { call, put, select, takeLatest, fork, all } from 'redux-saga/effects';
import {
  fetchUser,
  fetchRepo,
  fetchStarred,
  fetchStargazers
} from '../services/api';
import {
  getUser,
  getRepo,
  getStarredByUser,
  getSargazersByRepo
} from '../reducers/selectors';

import * as actions from '../actions';

function* loadUser(login, requiredFields) {
  const user = yield select(getUser, login);
  if (user && requiredFields.every(key => user.hasOwnProperty(key))) {
    return null;
  }

  yield put(actions.user.request());
  const { response, error } = yield call(fetchUser, login);
  if (response) {
    yield put(actions.user.success(response));
  } else {
    yield put(actions.user.failure(error));
  }
}

function* loadRepo(fullName, requiredFields) {
  const repo = yield select(getRepo, fullName);
  if (repo && requiredFields.every(key => repo.hasOwnProperty(key))) {
    return null;
  }

  yield put(actions.repo.request());
  const { response, error } = yield call(fetchRepo, fullName);
  if (response) {
    yield put(actions.repo.success(response));
  } else {
    yield put(actions.repo.failure(error));
  }
}

function* loadStarred(login, nextPage) {
  const starredByUser = yield select(getStarredByUser, login);
  const {
    nextPageUrl = `users/${login}/starred`,
    pageCount = 0
  } = starredByUser;
  if (pageCount > 0 && !nextPage) {
    return null;
  }

  yield put(actions.starred.request(login));
  const { response, error } = yield call(fetchStarred, nextPageUrl);
  if (response) {
    yield put(actions.starred.success(login, response));
  } else {
    yield put(actions.starred.failure(login, error));
  }
}

function* loadStargazers(fullName, nextPage) {
  const stargazersByRepo = yield select(getSargazersByRepo, fullName);
  const {
    nextPageUrl = `repos/${fullName}/stargazers`,
    pageCount = 0
  } = stargazersByRepo;
  if (pageCount > 0 && !nextPage) {
    return null;
  }

  yield put(actions.stargazers.request(fullName));
  const { response, error } = yield call(fetchStargazers, nextPageUrl);
  if (response) {
    yield put(actions.stargazers.success(fullName, response));
  } else {
    yield put(actions.stargazers.failure(fullName, error));
  }
}

/******************************************************************************/
/******************************* WATCHERS *************************************/
/******************************************************************************/

function* watchLoadUserPageData(action) {
  const { login, requiredFields = [] } = action;
  yield fork(loadUser, login, requiredFields);
  yield fork(loadStarred, login);
}

function* watchLoadRepoPageData(action) {
  const { fullName, requiredFields = [] } = action;
  yield fork(loadRepo, fullName, requiredFields);
  yield fork(loadStargazers, fullName);
}

function* watchLoadStarred(action) {
  const { login, nextPage } = action;
  yield fork(loadStarred, login, nextPage);
}

function* watchLoadStargazers(action) {
  const { fullName, nextPage } = action;
  yield fork(loadStargazers, fullName, nextPage);
}

export default function* root() {
  yield all([
    takeLatest(actions.LOAD_USER_PAGE_DATA, watchLoadUserPageData),
    takeLatest(actions.LOAD_REPO_PAGE_DATA, watchLoadRepoPageData),
    takeLatest(actions.LOAD_STARRED, watchLoadStarred),
    takeLatest(actions.LOAD_STARGAZERS, watchLoadStargazers)
  ]);
}
