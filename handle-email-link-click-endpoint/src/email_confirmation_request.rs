use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EmailConfirmationServiceApiResponse {
    pub error: bool,
    pub request: SanitizedEmailConfirmationRequest
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SanitizedEmailConfirmationRequest {
    pub pk: String,
    pub email: String,
    pub client_id: String,
    pub request_id: String,
    pub callback_url: String,
    pub expires_at: u64,
    pub status: Status,
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