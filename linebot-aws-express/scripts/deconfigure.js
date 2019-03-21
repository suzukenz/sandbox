#!/usr/bin/env node
'use strict'

const modifyFiles = require('./utils').modifyFiles
const packageJson = require('../package.json')
const config = packageJson.config

modifyFiles(['./simple-proxy-api.yaml', './package.json', './cloudformation.yaml'], [{
  regexp: new RegExp(config.accountId, 'g'),
  replacement: 'YOUR_ACCOUNT_ID'
}, {
  regexp: new RegExp(config.region, 'g'),
  replacement: 'YOUR_AWS_REGION'
}, {
  regexp: new RegExp(config.s3BucketName, 'g'),
  replacement: 'YOUR_UNIQUE_BUCKET_NAME'
}, {
  regexp: new RegExp(config.functionName, 'g'),
  replacement: 'YOUR_SERVERLESS_EXPRESS_LAMBDA_FUNCTION_NAME'
}, {
  regexp: new RegExp(config.enableScheduled, 'g'),
  replacement: 'YOUR_LAMBDA_FUNCTION_IS_SCHEDULED_OR_NOT'
}, {
  regexp: new RegExp(escapeRegExp(config.scheduleExpression), 'g'),
  replacement: 'YOUR_LAMBDA_FUNCTION_SCHEDULE_EXPRESSION'
}])

function escapeRegExp(string){
  return string.replace(/([.*+?^=!:${}()|[\]/\\])/g, '\\$1')
}