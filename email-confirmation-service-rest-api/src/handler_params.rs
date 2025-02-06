use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use crate::email_confirmation_request;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueryParams {
    pub email: Option<String>,
    pub client_id: Option<String>,
    pub request_id: Option<String>,
    pub expires_after: Option<SystemTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PutStatusParams {
    pub status: email_confirmation_request::Status,
}
