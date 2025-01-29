use std::ops::Add;
use serde::{Deserialize, Serialize};
use std::time::*;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmailConfirmationMinimalRequest {
    pub email: String,
    pub client_id: String,
    pub callback_url: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmailConfirmationRequest {
    pub id: String,
    pub email: String,
    pub client_id: String,
    pub callback_url: String,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub status: Status,
}

impl EmailConfirmationRequest {
    pub fn new(email: String, client_id: String, callback_url: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let created_at = SystemTime::now();
        let expires_at = SystemTime::now().add(Duration::from_secs(60 * 60));

        EmailConfirmationRequest { id, email, client_id, callback_url, created_at, expires_at, status: Status::None }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum MinimalStatus {
    None,
    Queued,
    Pending,
    Confirmed,
    Expired,
    Done
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Status {
    None,
    Queued(SystemTime),
    Pending(SystemTime),
    Confirmed(SystemTime),
    Expired(SystemTime),
    Done(SystemTime)
}

impl From<&MinimalStatus> for Status {
    fn from(status: &MinimalStatus) -> Self {
        match *status {
            MinimalStatus::None => Status::None,
            MinimalStatus::Queued=> Status::Queued(SystemTime::now()),
            MinimalStatus::Pending=> Status::Pending(SystemTime::now()),
            MinimalStatus::Confirmed=> Status::Confirmed(SystemTime::now()),
            MinimalStatus::Expired=> Status::Expired(SystemTime::now()),
            MinimalStatus::Done=> Status::Done(SystemTime::now())
        }
    }
}
