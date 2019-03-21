'use strict'
const awsServerlessExpress = require(process.env.NODE_ENV === 'test' ? '../src/index' : 'aws-serverless-express')
const app = require('./app')

const server = awsServerlessExpress.createServer(app)

exports.handler = (event, context) => awsServerlessExpress.proxy(server, event, context)
