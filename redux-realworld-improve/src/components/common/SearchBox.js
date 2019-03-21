import React from 'react';
import PropTypes from 'prop-types';

const SearchBox = ({ inputValue, onClick, onChange, onKeyUp }) => {
  return (
    <div>
      <input
        size="45"
        value={inputValue}
        onChange={onChange}
        onKeyUp={onKeyUp}
      />
      <button onClick={onClick}>Go!</button>
    </div>
  );
};

SearchBox.propTypes = {
  inputValue: PropTypes.string.isRequired,
  onClick: PropTypes.func.isRequired,
  onChange: PropTypes.func.isRequired,
  onKeyUp: PropTypes.func.isRequired
};

export default SearchBox;
