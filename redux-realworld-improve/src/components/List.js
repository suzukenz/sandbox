import React from 'react';
import PropTypes from 'prop-types';

const LoadMore = ({ isFetching, onClick }) => {
  return (
    <button
      style={{ fontSize: '150%' }}
      onClick={onClick}
      disabled={isFetching}
    >
      {isFetching ? 'Loading...' : 'Load More'}
    </button>
  );
};
LoadMore.propTypes = {
  isFetching: PropTypes.bool.isRequired,
  onClick: PropTypes.func.isRequired
};

const List = props => {
  const {
    isFetching,
    isEmpty,
    nextPageUrl,
    pageCount,
    loadingLabel,
    onLoadMoreClick,
    children
  } = props;

  if (isEmpty && isFetching) {
    return (
      <h2>
        <i>{loadingLabel}</i>
      </h2>
    );
  }

  const isLastPage = !nextPageUrl;
  if (isEmpty && isLastPage) {
    return (
      <h1>
        <i>Nothing here!</i>
      </h1>
    );
  }

  return (
    <div>
      {children}
      {pageCount > 0 && !isLastPage ? (
        <LoadMore isFetching={isFetching} onClick={onLoadMoreClick} />
      ) : null}
    </div>
  );
};

List.propTypes = {
  loadingLabel: PropTypes.string.isRequired,
  pageCount: PropTypes.number,
  isFetching: PropTypes.bool.isRequired,
  isEmpty: PropTypes.bool.isRequired,
  onLoadMoreClick: PropTypes.func.isRequired,
  nextPageUrl: PropTypes.string,
  children: PropTypes.node.isRequired
};

List.defaultProps = {
  isFetching: true,
  loadingLabel: 'Loading...'
};

export default List;
