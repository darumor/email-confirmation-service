use anyhow::Result;
use axum::http::{StatusCode};
use axum::{
    extract::{Path, State, Query},
    response::Json,
};
use serde_json::{json, Value};
use crate::email_confirmation_request::{EmailConfirmationMinimalRequest, EmailConfirmationRequest};
use crate::email_confirmation_request_service::{EmailConfirmationRequestService, NOT_FOUND_ERROR};
use crate::handler_params::{PutStatusParams, QueryParams};


pub async fn get_email_confirmation_requests(
    State(service): State<EmailConfirmationRequestService>,
    Query(params): Query<QueryParams>,
) -> (StatusCode, Json<Value>) {
    let result = service.get_email_confirmation_requests(params).await;
    result_to_response(result)
}

pub async fn post_email_confirmation_request(
    State(service): State<EmailConfirmationRequestService>,
    Json(minimal_request): Json<EmailConfirmationMinimalRequest>,
) -> (StatusCode, Json<Value>) {
    let request = EmailConfirmationRequest::from(minimal_request);
    let result = service.post_email_confirmation_request(request).await;
    result_to_response(result)
}

pub async fn get_email_confirmation_request_single(
    State(service): State<EmailConfirmationRequestService>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    let result = service.get_email_confirmation_request_single(id).await;
    result_to_response(result)
}

pub async fn delete_email_confirmation_request_single(
    State(service): State<EmailConfirmationRequestService>,
    Path(pk): Path<String>,
) -> (StatusCode, Json<Value>) {
    let result = service.delete_email_confirmation_request_single(pk).await;
    result_to_response(result)
}

pub async fn put_email_confirmation_request_status(
    State(service): State<EmailConfirmationRequestService>,
    Path(pk): Path<String>,
    Json(put_status_params): Json<PutStatusParams>,
) -> (StatusCode, Json<Value>) {
    let result = service.put_email_confirmation_request_status(pk, put_status_params.status).await;
    result_to_response(result)
}

fn result_to_response(result: Result<Json<Value>>) -> (StatusCode, Json<Value>) {
    match result {
        Ok(json) => (StatusCode::OK, json),
        Err(error) => {
            let mut err_str = String::new();
            error.chain().skip(1).for_each(
                |cause| {
                    err_str.push_str(" because: ");
                    err_str.push_str(cause.to_string().as_str());
                }
            );

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": true,
                    "message": err_str
                }))
            )
        }

/*
        if error.to_string().starts_with(NOT_FOUND_ERROR) {
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                "error": true,
                "message": error.to_string()
            })),
            )
        } else {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                "error": true,
                "message": error.to_string()
            })),
            )
        }

 */
    }
}

