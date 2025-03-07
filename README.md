# email-confirmation-service
This is a project for practicing using rust to create aws lambdas

The project was started by following [this tutorial](https://blog.stackademic.com/rust-apigateway-lambda-dynamo-cdk-another-all-in-one-serverless-backend-option-4da2059a8810)

## Prerequisites
- aws-cdk
- aws account

## Technologies
- Rust
- Cargo-Lambda
- Axum
- Dynamo

## Environment variables to be set
_See setup-environment-template.sh_

### EmailConfirmationLambdaFunction
- EMAIL_CONFIRMATION_REQUEST_SERVICE_DYNAMO_TABLE_NAME
- SIGNATURE_SERVICE_LAMBDA_FUNCTION_NAME

### HandleEmailLinkClickLambdaFunction
- EMAIL_CONFIRMATION_REQUEST_SERVICE_URL
- EMAIL_CONFIRMATION_REQUEST_SERVICE_INTERNAL_API_KEY

### SendEmailEventLambdaFunction
- EMAIL_CONFIRMATION_DYNAMODB_STREAM_ARN

### SignatureServiceLambdaFunction
- EMAIL_CONFIRMATION_SERVICE_LAMBDA_ARN
- EMAIL_SENDING_LAMBDA_ARN

### TriggerCallbackEventLambdaFunction
- EMAIL_CONFIRMATION_DYNAMODB_STREAM_ARN


## Set up API keys
Create API keys in AWS. 
- One is needed for email-link-click-handler to call email-confirmation-request-service.
- Another one is needed for client for posting new email-confirmation-requests

API-key is sent to API Gateway in x-api-key header of the request.

## Service startup
    . setup-environment.sh 
    for each subapplication:
        cd subapplication/cdk
        cdk deploy

## Service usage
...



## Local testing
### Start service
    cd email-confirmation-service-rest-api
    cargo lambda start

### Smoke test
Go to [http://localhost:9000](http://localhost:9000)

### API testing
- use e.g. [Postman](https://www.postman.com)

