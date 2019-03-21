'use strict'
const line = require('@line/bot-sdk')

class LineClient {

  constructor() {
    // line bot secrets
    this.config = {
      channelAccessToken: 'xxxxxxxxxx',
      channelSecret: 'xxxxxxxxxx'
    }
    this.client = new line.Client(this.config)

    // user or group or room IDs
    this.pushTargets = [
      'target user id'
    ]
  }

  getMiddleware() {
    return line.middleware(this.config)
  }

  pushMessage(message) {
    return Promise.all(this.pushTargets.map(target => {
      return this.client.pushMessage(target, {type: 'text', text: message})
    }))
  }

  webhookHandler() {
    return event => {
      if (event.type !== 'message' || event.message.type !== 'text') {
        return Promise.resolve(null)
      }
    
      const msg = event.message.text
    
      let repMsg = msg
      switch (msg) {
        case 'me':
          repMsg = 'Your user id is ' + event.source.userId
          break
        case 'group':
          repMsg = (event.source.type == 'group') ? 
            'This group id is ' + event.source.groupId : 
            'Fail to get group id'
          break
        case 'room':
          repMsg = (event.source.type == 'room') ? 
            'This room id is ' + event.source.roomId : 
            'Fail to get room id'
          break
      }
    
      console.log('replyMessage: ' + repMsg)
      return this.client.replyMessage(event.replyToken, {
        type: 'text',
        text: repMsg
      })
    }
  }
}

module.exports = LineClient