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

## Local testing
### Start service
    cd email-confirmation-service-rest-api
    cargo lambda start

### Smoke test
Go to [http://localhost:9000](http://localhost:9000)

### API testing
- use e.g. [Postman](https://www.postman.com)
