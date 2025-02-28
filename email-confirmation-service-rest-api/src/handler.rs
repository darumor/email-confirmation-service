use anyhow::{Error, Result};
use axum::http::{StatusCode};
use axum::{
    extract::{Path, State, Query},
    response::Json,
};
use serde_json::{json, Value};
use sha2::{Sha256, Digest};
use hex;
use crate::email_confirmation_request::{EmailConfirmationMinimalRequest, EmailConfirmationRequest, SanitizedEmailConfirmationRequest};
use crate::email_confirmation_request_service::{EmailConfirmationRequestService, NOT_FOUND_ERROR};
use crate::handler_params::{GetSingleParams, PutStatusParams, QueryParams};


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
    let signature = create_signature(&request);
    let result = service.post_email_confirmation_request(request, signature).await;
    result_to_response(result)
}

pub async fn get_email_confirmation_request_single(
    State(service): State<EmailConfirmationRequestService>,
    Path(pk): Path<String>,
    Query(params): Query<GetSingleParams>,
) -> (StatusCode, Json<Value>) {
    if let GetSingleParams {
        signature: Some(signature_param)
    } = params {
        let confirmation_request = service.get_email_confirmation_request_internal(pk).await.unwrap();

        if signature_is_valid(signature_param, &confirmation_request) {
            return result_to_response(
                Ok(Json(json!({
                    "error": false,
                    "request": SanitizedEmailConfirmationRequest::from(confirmation_request)
                }))));
        }
    }
    result_to_response(Err(Error::msg(NOT_FOUND_ERROR.to_string())))
}

fn signature_is_valid(signature: String, confirmation_request: &EmailConfirmationRequest) -> bool {
    signature == create_signature(confirmation_request)
}

fn create_signature(confirmation_request: &EmailConfirmationRequest) -> String {
    let email = confirmation_request.email.clone();
    let client_id = confirmation_request.client_id.clone();
    let request_id = confirmation_request.request_id.clone();
    let signature_key = confirmation_request.signature_key.clone();
    let updated_at = confirmation_request.updated_at;

    let data = format!("|{}|{}|{}|{}|{}|", email, client_id, request_id, signature_key, updated_at);
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let signature = hex::encode(result);

    signature
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
    if let PutStatusParams {
        status: Some(status_param),
        signature: Some(signature_param)
    } = put_status_params {
        let confirmation_request = service.get_email_confirmation_request_internal(pk.clone()).await.unwrap();
        if signature_is_valid(signature_param, &confirmation_request) {
            let updated_request = service.put_email_confirmation_request_status(pk.clone(), status_param).await.unwrap();
            return result_to_response(
                Ok(Json(json!({
                        "error": false,
                        "request": SanitizedEmailConfirmationRequest::from(updated_request)
                    }))));
        }
    }
    result_to_response(Err(Error::msg(NOT_FOUND_ERROR.to_string())))
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
    }
}

