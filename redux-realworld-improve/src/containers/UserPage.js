/* eslint-disable react/no-deprecated */

import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import { withRouter } from 'react-router-dom';
import { compose } from 'recompose';
import { loadUserPageData, loadStarred } from '../actions';
import User from '../components/User';
import Repo from '../components/Repo';
import List from '../components/List';
import zip from 'lodash/zip';

const requiredFields = ['name'];

class UserPage extends Component {
  static propTypes = {
    login: PropTypes.string.isRequired,
    user: PropTypes.object,
    starredPagination: PropTypes.object,
    starredRepos: PropTypes.array.isRequired,
    starredRepoOwners: PropTypes.array.isRequired,
    loadUserPageData: PropTypes.func.isRequired,
    loadStarred: PropTypes.func.isRequired
  };

  componentWillMount() {
    this.props.loadUserPageData(this.props.login, requiredFields);
  }

  componentWillReceiveProps(nextProps) {
    if (nextProps.login !== this.props.login) {
      this.props.loadUserPageData(nextProps.login, requiredFields);
    }
  }

  handleLoadMoreClick = () => {
    this.props.loadStarred(this.props.login, true);
  };

  render() {
    const { user, login } = this.props;
    if (!user) {
      return (
        <h1>
          <i>
            Loading {login}
            {"'s profile..."}
          </i>
        </h1>
      );
    }

    const { starredRepos, starredRepoOwners, starredPagination } = this.props;
    const items = zip(starredRepos, starredRepoOwners);
    return (
      <div>
        <User user={user} />
        <hr />
        <List
          isEmpty={items.length === 0}
          onLoadMoreClick={this.handleLoadMoreClick}
          loadingLabel={`Loading ${login}'s starred...`}
          {...starredPagination}
        >
          {items.map(([repo, owner]) => (
            <Repo repo={repo} owner={owner} key={repo.fullName} />
          ))}
        </List>
      </div>
    );
  }
}

const mapStateToProps = (state, ownProps) => {
  // We need to lower case the login due to the way GitHub's API behaves.
  // Have a look at ../middleware/api.js for more details.
  const login = ownProps.match.params.login.toLowerCase();

  const {
    pagination: { starredByUser },
    entities: { users, repos }
  } = state;

  const starredPagination = starredByUser[login] || { ids: [] };
  const starredRepos = starredPagination.ids.map(id => repos[id]);
  const starredRepoOwners = starredRepos.map(repo => users[repo.owner]);

  return {
    login,
    starredRepos,
    starredRepoOwners,
    starredPagination,
    user: users[login]
  };
};

const enhance = compose(
  withRouter,
  connect(
    mapStateToProps,
    {
      loadUserPageData,
      loadStarred
    }
  )
);

export default enhance(UserPage);
