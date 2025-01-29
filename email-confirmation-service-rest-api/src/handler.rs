use lambda_http::{Body, Error, Request, RequestExt, Response};

use anyhow::Result;
use axum::extract::Query;
use axum::http::{request, StatusCode};
use axum::{
    extract::{Path, State},
    response::Json,
};
use serde_json::{json, Value};
use crate::email_confirmation_request;
use crate::email_confirmation_request::{EmailConfirmationMinimalRequest, EmailConfirmationRequest};
use crate::email_confirmation_request_service::EmailConfirmationRequestService;
use crate::handler_params::{PutStatusParams, QueryParams};


pub async fn get_email_confirmation_requests(
    State(service): State<EmailConfirmationRequestService>,
    Query(params): Query<QueryParams>,
) -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({ "request parameters": params })),
    )
}

pub async fn post_email_confirmation_request(
    State(service): State<EmailConfirmationRequestService>,
    Json(request): Json<EmailConfirmationMinimalRequest>,
) -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({ "request body": request })),
    )
}

pub async fn get_email_confirmation_request_single(
    State(service): State<EmailConfirmationRequestService>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({ "get request id": id})),
    )
}

pub async fn delete_email_confirmation_request_single(
    State(service): State<EmailConfirmationRequestService>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({ "delete request id": id })),
    )
}

pub async fn put_email_confirmation_request_status(
    State(service): State<EmailConfirmationRequestService>,
    Path(id): Path<String>,
    Json(put_status_params): Json<PutStatusParams>,
) -> (StatusCode, Json<Value>) {
    let request_status= email_confirmation_request::Status::from(&put_status_params.status);
    (
        StatusCode::OK,
        Json(json!({ "put new status": put_status_params.status, "set status" : request_status })),
    )
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use lambda_http::{Request, RequestExt};

    #[tokio::test]
    async fn test_get_email_confirmation_requests() {
        let request = Request::default();

        let response = get_email_confirmation_requests().await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_http_handler_with_query_string() {
        let mut query_string_parameters: HashMap<String, String> = HashMap::new();
        query_string_parameters.insert("name".into(), "email-confirmation-service-rest-api".into());

        let request = Request::default()
            .with_query_string_parameters(query_string_parameters);

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello email-confirmation-service-rest-api, this is an AWS Lambda HTTP request"
        );
    }
}*/
