use anyhow::{bail, Ok, Result};
use aws_sdk_dynamodb::Client;
use axum::Json;
use serde_json::Value;
use crate::email_confirmation_request::{EmailConfirmationRequest, MinimalStatus, Status};
use crate::handler_params::QueryParams;

#[derive(Clone, Debug)]
pub struct EmailConfirmationRequestService {
    db_client: Client,
    table_name: String,
}

impl EmailConfirmationRequestService {
    pub fn new(db_client: Client, table_name: &str) -> Self {
        Self {
            db_client,
            table_name: table_name.to_owned(),
        }
    }

    pub async fn get_email_confirmation_requests(&self, params: QueryParams) -> Result<Json<Value>> {
        // ...
        Ok(Json(Value::String("{\"function\":\"get_email_confirmation_requests\"}".to_string())))
    }

    pub async fn post_email_confirmation_request(&self, event: EmailConfirmationRequest) -> Result<Json<Value>> {
        Ok(Json(Value::String("{\"function\":\"post_email_confirmation_request\"}".to_string())))
        // ...
    }

    pub async fn get_email_confirmation_request_single(&self, id: String) -> Result<Json<Value>> {
        Ok(Json(Value::String("{\"function\":\"get_email_confirmation_request_single\"}".to_string())))
        // ...
    }

    pub async fn delete_email_confirmation_request_single(&self, id: String) -> Result<Json<Value>> {
        Ok(Json(Value::String("{\"function\":\"delete_email_confirmation_request_single\"}".to_string())))
        // ...
    }

    pub async fn put_email_confirmation_request_status(&self, id: String, status: Status) -> Result<Json<Value>> {
        Ok(Json(Value::String("{\"function\":\"put_email_confirmation_request_status\"}".to_string())))
        // ...
    }
}