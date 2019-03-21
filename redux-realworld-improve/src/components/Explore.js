import React from 'react';
import PropTypes from 'prop-types';
import { compose, withState, withHandlers, lifecycle, pure } from 'recompose';

import SearchBox from './common/SearchBox';

const GITHUB_REPO = 'https://github.com/reactjs/redux';

const SearchBoxEnhance = compose(
  withState('inputValue', 'setValue', ''),
  withHandlers({
    onChange: ({ setValue }) => event => {
      setValue(event.target.value);
    },
    onClick: ({ inputValue, onClick }) => () => {
      onClick(inputValue);
    },
    onKeyUp: ({ inputValue, onClick }) => event => {
      if (event.keyCode === 13) {
        onClick(inputValue);
      }
    }
  }),
  lifecycle({
    componentWillMount() {
      this.props.setValue(this.props.defaultValue);
    },
    componentWillReceiveProps(nextProps) {
      if (nextProps.defaultValue !== this.props.defaultValue) {
        this.props.setValue(nextProps.defaultValue);
      }
    }
  }),
  pure
);
const MySearchBox = SearchBoxEnhance(SearchBox);

const Explore = ({ value, onChange }) => {
  return (
    <div>
      <p>Type a username or repo full name and hit &apos;Go&apos;:</p>
      <MySearchBox defaultValue={value} onClick={onChange} />
      <p>
        Code on{' '}
        <a href={GITHUB_REPO} target="_blank" rel="noopener noreferrer">
          Github
        </a>
        .
      </p>
      <p>Move the DevTools with Ctrl+W or hide them with Ctrl+H.</p>
    </div>
  );
};

Explore.propTypes = {
  value: PropTypes.string.isRequired,
  onChange: PropTypes.func.isRequired
};

export default Explore;
