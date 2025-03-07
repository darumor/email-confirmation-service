# email-confirmation-service
This is a project for practicing using rust and aws-cdk to create aws lambdas. 

## Descrition of the application
The main concept is that a client can request this service to confirm authenticity of an email address. 
After a request is made the service creates a link that is sent to the email using AWS SES. 
When recipient clicks the link the address is confirmed and a POST is made to the requested callback url. 
The link is protected with a signature that is created using information that is never sent outside 
the service and it expires in 60 minutes. (The lifetime could be an environment variable too)

This project uses different ways to invoke lambdas: API Gateway, direct invocation DynamoDB streams (and probably SNS or SQS). 

The project was started by following [this tutorial](https://blog.stackademic.com/rust-apigateway-lambda-dynamo-cdk-another-all-in-one-serverless-backend-option-4da2059a8810)

## Prerequisites
- aws-cdk
- aws account

## Technologies
- Rust
- Cargo-Lambda
- Axum
- Dynamo
- API Gateway
- AWS SES

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


## Testing

### Unit tests

### Local testing

### API testing
- use e.g. [Postman](https://www.postman.com)

