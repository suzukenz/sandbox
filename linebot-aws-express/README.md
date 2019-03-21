
Example application for Line Bot on aws serverless.

This app is based on [aws-serverless-express](https://github.com/awslabs/aws-serverless-express).

## Usage
### Configure
```
npm run config -- \
  --account-id="<accountId>" \
  --bucket-name="<bucketName>" \
  [--region="<region>" \
  --function-name="<functionName>" \
  --schedule-expression='<lambdaScheduleExpression>']
```
This modifies `package.json`, `simple-proxy-api.yaml` and `cloudformation.yaml`

### Deploy
```
npm run setup
```
this installs the node dependencies, creates an S3 bucket (if it does not already exist), packages and deploys your serverless Express application to AWS Lambda, and creates an API Gateway proxy API.
