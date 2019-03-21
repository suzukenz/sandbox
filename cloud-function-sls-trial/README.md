# cloud-function-trial

Try Google Cloud Function with Serverless Framework

ref: [Serverss Framework official docs](https://serverless.com/framework/docs/providers/google/)

## Setup

1. Run `npm install`
2. Update the `project` and `credentials` in `serverless.yml`

See the [official docs](https://serverless.com/framework/docs/providers/google/guide/credentials/) for more info about credentials.

## Usage

### Deploy

```
npm run deploy
```

### Undeploy

```
npm run undeploy
```

### Run function (after deploying)

```
npm run start <function-name>
```

ex) `npm run start hello`

### View logs

```
npm run logs <function-name>
```

ex) `npm run logs hello`

### Run local

**This command is not work now**

```
npm run local <function-name>
```

### Use serverless commands on manual

```
npm run sls -- commands
```

ex) `npm run sls -- help`
