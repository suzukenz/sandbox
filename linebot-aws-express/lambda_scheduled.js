'use strict'
const LineClient = require('./line-client')

exports.handler = (event, context, callback) => {
  const line = new LineClient()
  line.pushMessage('hello').then(() => {
    return callback(null, 'success')
  }).catch(err => {
    console.log('Caught Error: ', err)
    callback(err)
  })
}
