use serde::{Deserialize, Serialize};
use email_confirmation_service_common::email_confirmation_request;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueryParams {
    pub email: Option<String>,
    pub client_id: Option<String>,
    pub request_id: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PutStatusParams {
    pub status: Option<email_confirmation_request::Status>,
    pub signature: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetSingleParams {
    pub signature: Option<String>
}
