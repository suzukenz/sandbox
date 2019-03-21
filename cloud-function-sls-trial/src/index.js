const req = require('request');

exports.helloworld = (request, response) => {
  console.log('write logs'); // view by using "logs" command
  response.status(200).send('hello world');
};

exports.repos = (request, response) => {
  req(
    {
      url: 'https://api.github.com/users/suzukenz/repos',
      headers: {
        'User-Agent': 'cloud-function-trial-app'
      }
    },
    (error, res, body) => {
      response.status(200).send(body);
    }
  );
};
