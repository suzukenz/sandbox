/* eslint-disable react/no-deprecated */

import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import { withRouter } from 'react-router-dom';
import { compose } from 'recompose';
import { loadRepoPageData, loadStargazers } from '../actions';
import Repo from '../components/Repo';
import User from '../components/User';
import List from '../components/List';

const requiredFields = ['description'];

class RepoPage extends Component {
  static propTypes = {
    repo: PropTypes.object,
    fullName: PropTypes.string.isRequired,
    name: PropTypes.string.isRequired,
    owner: PropTypes.object,
    stargazers: PropTypes.array.isRequired,
    stargazersPagination: PropTypes.object,
    loadRepoPageData: PropTypes.func.isRequired,
    loadStargazers: PropTypes.func.isRequired
  };

  componentWillMount() {
    this.props.loadRepoPageData(this.props.fullName, requiredFields);
  }

  componentWillReceiveProps(nextProps) {
    if (nextProps.fullName !== this.props.fullName) {
      this.props.loadRepoPageData(nextProps.fullName, requiredFields);
    }
  }

  handleLoadMoreClick = () => {
    this.props.loadStargazers(this.props.fullName, true);
  };

  render() {
    const { repo, owner, name } = this.props;
    if (!repo || !owner) {
      return (
        <h1>
          <i>Loading {name} details...</i>
        </h1>
      );
    }

    const { stargazers, stargazersPagination } = this.props;
    return (
      <div>
        <Repo repo={repo} owner={owner} />
        <hr />
        <List
          isEmpty={stargazers.length === 0}
          onLoadMoreClick={this.handleLoadMoreClick}
          loadingLabel={`Loading stargazers of ${name}...`}
          {...stargazersPagination}
        >
          {stargazers.map(user => (
            <User user={user} key={user.login} />
          ))}
        </List>
      </div>
    );
  }
}

const mapStateToProps = (state, ownProps) => {
  // We need to lower case the login/name due to the way GitHub's API behaves.
  // Have a look at ../middleware/api.js for more details.
  const login = ownProps.match.params.login.toLowerCase();
  const name = ownProps.match.params.name.toLowerCase();

  const {
    pagination: { stargazersByRepo },
    entities: { users, repos }
  } = state;

  const fullName = `${login}/${name}`;
  const stargazersPagination = stargazersByRepo[fullName] || { ids: [] };
  const stargazers = stargazersPagination.ids.map(id => users[id]);

  return {
    fullName,
    name,
    stargazers,
    stargazersPagination,
    repo: repos[fullName],
    owner: users[login]
  };
};

const enhance = compose(
  withRouter,
  connect(
    mapStateToProps,
    {
      loadRepoPageData,
      loadStargazers
    }
  )
);

export default enhance(RepoPage);
