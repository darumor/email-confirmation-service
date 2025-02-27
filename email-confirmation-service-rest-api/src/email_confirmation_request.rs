use std::fmt;
use std::ops::Add;
use serde::{Deserialize, Serialize};
use std::time::*;
use uuid::Uuid;
use crate::handler_params::QueryParams;

pub const EMAIL_REQUEST_EXPIRATION_PERIOD:Duration = Duration::from_secs(60 * 60);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmailConfirmationMinimalRequest {
    pub email: String,
    pub client_id: String,
    pub request_id: String,
    pub callback_url: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EmailConfirmationRequest {
    pub pk: String, //PK email#client_id#request_id
    pub email: String,
    pub client_id: String,
    pub request_id: String,
    pub callback_url: String,
    pub signature_key: String,
    pub created_at: u64,
    pub expires_at: u64, // SK
    pub updated_at: u64,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SanitizedEmailConfirmationRequest {
    pub pk: String,
    pub email: String,
    pub client_id: String,
    pub request_id: String,
    pub callback_url: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub updated_at: u64,
    pub status: Status,
}

impl From<EmailConfirmationMinimalRequest> for EmailConfirmationRequest {
    fn from(minimal_request: EmailConfirmationMinimalRequest) -> Self {
        EmailConfirmationRequest::new(minimal_request.email, minimal_request.client_id, minimal_request.request_id, minimal_request.callback_url)
    }
}

impl From<EmailConfirmationRequest> for SanitizedEmailConfirmationRequest {
    fn from(original_request: EmailConfirmationRequest) -> Self {
        SanitizedEmailConfirmationRequest{
            pk: original_request.pk,
            email: original_request.email,
            client_id: original_request.client_id,
            request_id: original_request.request_id,
            callback_url: original_request.callback_url,
            created_at: original_request.created_at,
            expires_at: original_request.expires_at,
            updated_at: original_request.updated_at,
            status: original_request.status,
        }
    }
}
impl EmailConfirmationRequest {
    pub fn new(email: String, client_id: String, request_id: String, callback_url: String) -> Self {
        let pk = Self::pk_from_params(&email, &client_id, &request_id);
        let signature_key = Uuid::new_v4().to_string();
        let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let expires_at = SystemTime::now().add(EMAIL_REQUEST_EXPIRATION_PERIOD).duration_since(UNIX_EPOCH).unwrap().as_secs();
        let updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        EmailConfirmationRequest { pk, email, client_id, request_id, callback_url, signature_key, created_at, expires_at, updated_at, status: Status::Queued }
    }

    pub fn pk_from_query_params (params: &QueryParams) -> Result<String, String> {
        if let QueryParams {
            email: Some(email_param),
            client_id: Some(client_id_param),
            request_id: Some(request_id_param),
            expires_after: _
        } = params {
            return Ok(Self::pk_from_params(email_param, client_id_param, request_id_param));
        }
        Err(format!("Invalid parameters {params:?}"))
    }

    pub fn pk_from_params (email: &str, client_id: &str, request_id: &str) -> String {
        format!("{email}#{client_id}#{request_id}")
    }

}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Status {
    Queued,
    Pending,
    Confirmed,
    Expired,
    Done
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Queued => write!(f, "Queued"),
            Status::Pending => write!(f, "Pending"),
            Status::Confirmed => write!(f, "Confirmed"),
            Status::Expired => write!(f, "Expired"),
            Status::Done => write!(f, "Done"),
        }
    }
}