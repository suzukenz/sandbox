'use strict'
const express = require('express')
const bodyParser = require('body-parser')
const cors = require('cors')
const compression = require('compression')
const awsServerlessExpressMiddleware = require('aws-serverless-express/middleware')
const LineClient = require('./line-client')

const app = express()
const router = express.Router()
const line = new LineClient()

if (process.env.NODE_ENV === 'test') {
  // NOTE: aws-serverless-express uses this app for its integration tests
  // and only applies compression to the /sam endpoint during testing.
  router.use('/sam', compression())
} else {
  router.use(compression())
}

router.use(cors())
router.use(awsServerlessExpressMiddleware.eventContext())

router.post('/line/webhook', line.getMiddleware(), (req, res) => {
  Promise
    .all(req.body.events.map(line.webhookHandler()))
    .then((result) => res.json(result))
})
// bodyParser must be used after line.middleware
router.use(bodyParser.json())
router.use(bodyParser.urlencoded({ extended: true }))

router.get('/line/hello', (req, res) => {
  line.pushMessage('hello').then(() => {
    res.send('success send message')
  }).catch(err => {
    console.log(err)
    res.status(500).send('fail line send')
  })
})

// The aws-serverless-express library creates a server and listens on a Unix
// Domain Socket for you, so you can remove the usual call to app.listen.
// app.listen(3000)
app.use('/', router)

// Export your express server so you can import it in the lambda function.
module.exports = app
