use std::env;
use lambda_runtime::{run, service_fn, tracing, Error};

mod event_handler;
use event_handler::function_handler;


#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let stream_arn = env::var("EMAIL_CONFIRMATION_DYNAMODB_STREAM_ARN")?;
    // export EMAIL_CONFIRMATION_DYNAMODB_STREAM_ARN=arn:aws:dynamodb:eu-north-1:626635435572:table/EcrsStack-EmailConfirmationLambdaTable901FE3F0-1TVBZ9FZJB2AT/stream/2025-02-18T09:37:56.493

    run(service_fn(function_handler)).await
}
