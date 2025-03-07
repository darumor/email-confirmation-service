use lambda_http::{run, tracing, Error};
mod handler;
mod email_confirmation_request;
mod handler_params;
mod email_confirmation_request_service;
mod signature_request;

use std::env::{self, set_var};
use aws_sdk_dynamodb::Client;
use axum::Router;
use axum::routing::{get, put};
use crate::email_confirmation_request_service::EmailConfirmationRequestService;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    let config = aws_config::load_from_env().await;
    let db_client = Client::new(&config);
    let table_name = env::var("EMAIL_CONFIRMATION_REQUEST_SERVICE_DYNAMO_TABLE_NAME")?;

    let email_confirmation_request_service = EmailConfirmationRequestService::new(db_client, &table_name);
    let email_confirmation_request_api = Router::new()
        .route("/", get(handler::get_email_confirmation_requests).post(handler::post_email_confirmation_request))
        .route(
            "/{pk}",
            get(handler::get_email_confirmation_request_single).delete(handler::delete_email_confirmation_request_single),
        )
        .route("/{pk}/status", put(handler::put_email_confirmation_request_status));

    let app = Router::new()
        .nest("/email-confirmation-requests", email_confirmation_request_api)
        .with_state(email_confirmation_request_service);

    run(app).await
}
