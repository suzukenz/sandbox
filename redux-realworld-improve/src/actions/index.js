export const USER_REQUEST = 'USER_REQUEST';
export const USER_SUCCESS = 'USER_SUCCESS';
export const USER_FAILURE = 'USER_FAILURE';
export const user = {
  request: () => ({ type: USER_REQUEST }),
  success: response => ({ type: USER_SUCCESS, response }),
  failure: error => ({ type: USER_FAILURE, error })
};

export const LOAD_USER_PAGE_DATA = 'LOAD_USER_PAGE_DATA';
export const loadUserPageData = (login, requiredFields = []) => ({
  type: LOAD_USER_PAGE_DATA,
  login,
  requiredFields
});

export const REPO_REQUEST = 'REPO_REQUEST';
export const REPO_SUCCESS = 'REPO_SUCCESS';
export const REPO_FAILURE = 'REPO_FAILURE';
export const repo = {
  request: () => ({ type: REPO_REQUEST }),
  success: response => ({ type: REPO_SUCCESS, response }),
  failure: error => ({ type: REPO_FAILURE, error })
};

export const LOAD_REPO_PAGE_DATA = 'LOAD_REPO_PAGE_DATA';
export const loadRepoPageData = (fullName, requiredFields = []) => ({
  type: LOAD_REPO_PAGE_DATA,
  fullName,
  requiredFields
});

export const STARRED_REQUEST = 'STARRED_REQUEST';
export const STARRED_SUCCESS = 'STARRED_SUCCESS';
export const STARRED_FAILURE = 'STARRED_FAILURE';
export const starred = {
  request: login => ({ type: STARRED_REQUEST, login }),
  success: (login, response) => ({ type: STARRED_SUCCESS, login, response }),
  failure: (login, error) => ({ type: STARRED_FAILURE, login, error })
};

export const LOAD_STARRED = 'LOAD_STARRED';
export const loadStarred = (login, nextPage) => ({
  type: LOAD_STARRED,
  login,
  nextPage
});

export const STARGAZERS_REQUEST = 'STARGAZERS_REQUEST';
export const STARGAZERS_SUCCESS = 'STARGAZERS_SUCCESS';
export const STARGAZERS_FAILURE = 'STARGAZERS_FAILURE';
export const stargazers = {
  request: fullName => ({ type: STARGAZERS_REQUEST, fullName }),
  success: (fullName, response) => ({
    type: STARGAZERS_SUCCESS,
    fullName,
    response
  }),
  failure: (fullName, error) => ({ type: STARGAZERS_FAILURE, fullName, error })
};

export const LOAD_STARGAZERS = 'LOAD_STARGAZERS';
export const loadStargazers = (fullName, nextPage) => ({
  type: LOAD_STARGAZERS,
  fullName,
  nextPage
});

export const RESET_ERROR_MESSAGE = 'RESET_ERROR_MESSAGE';

// Resets the currently visible error message.
export const resetErrorMessage = () => ({
  type: RESET_ERROR_MESSAGE
});
