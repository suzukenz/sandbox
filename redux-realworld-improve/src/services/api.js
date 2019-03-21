import { normalize, schema } from 'normalizr';
import { camelizeKeys } from 'humps';

// Extracts the next page URL from Github API response.
const getNextPageUrl = response => {
  const link = response.headers.get('link');
  if (!link) {
    return null;
  }

  const nextLink = link.split(',').find(s => s.indexOf('rel="next"') > -1);
  if (!nextLink) {
    return null;
  }

  return nextLink
    .trim()
    .split(';')[0]
    .slice(1, -1);
};

const API_ROOT = 'https://api.github.com/';

// Fetches an API response and normalizes the result JSON according to schema.
// This makes every API response have the same shape, regardless of how nested it was.
const callApi = (endpoint, schema) => {
  const fullUrl =
    endpoint.indexOf(API_ROOT) === -1 ? API_ROOT + endpoint : endpoint;

  return fetch(fullUrl).then(response =>
    response
      .json()
      .then(json => {
        if (!response.ok) {
          return Promise.reject(json);
        }

        const camelizedJson = camelizeKeys(json);
        const nextPageUrl = getNextPageUrl(response);

        return Object.assign({}, normalize(camelizedJson, schema), {
          nextPageUrl
        });
      })
      .then(
        response => ({ response }),
        error => ({ error: error.message || 'Something bad happened' })
      )
  );
};

// We use this Normalizr schemas to transform API responses from a nested form
// to a flat form where repos and users are placed in `entities`, and nested
// JSON objects are replaced with their IDs. This is very convenient for
// consumption by reducers, because we can easily build a normalized tree
// and keep it updated as we fetch more data.

// Read more about Normalizr: https://github.com/paularmstrong/normalizr

// GitHub's API may return results with uppercase letters while the query
// doesn't contain any. For example, "someuser" could result in "SomeUser"
// leading to a frozen UI as it wouldn't find "someuser" in the entities.
// That's why we're forcing lower cases down there.

const userSchema = new schema.Entity(
  'users',
  {},
  {
    idAttribute: user => user.login.toLowerCase()
  }
);

const repoSchema = new schema.Entity(
  'repos',
  {
    owner: userSchema
  },
  {
    idAttribute: repo => repo.fullName.toLowerCase()
  }
);

const userArraySchema = [userSchema];
const repoArraySchema = [repoSchema];

export const fetchUser = login => callApi(`users/${login}`, userSchema);
export const fetchRepo = fullName => callApi(`repos/${fullName}`, repoSchema);
export const fetchStarred = url => callApi(url, repoArraySchema);
export const fetchStargazers = url => callApi(url, userArraySchema);
