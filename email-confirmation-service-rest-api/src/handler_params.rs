use serde::{Deserialize, Serialize};
use crate::email_confirmation_request;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueryParams {
    pub status: Option<email_confirmation_request::MinimalStatus>,
    pub email: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PutStatusParams {
    pub status: email_confirmation_request::MinimalStatus,
}
