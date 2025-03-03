use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SignatureRequest {
    pub signature_request_type: SignatureRequestType,
    pub signature_request_payload: SignatureRequestPayload,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum SignatureRequestType {
    SignatureCreationRequest,
    SignatureVerificationRequest
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum SignatureRequestPayload {
    SignatureCreationRequest(SignatureCreationData),
    SignatureVerificationRequest(SignatureVerificationData),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SignatureCreationData {
    pub email: String,
    pub client_id: String,
    pub request_id: String,
    pub updated_at: u64,
    pub signature_key: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SignatureVerificationData {
    pub email: String,
    pub client_id: String,
    pub request_id: String,
    pub updated_at: u64,
    pub signature_key: String,
    pub signature_value: String,
}

impl From<SignatureVerificationData> for SignatureCreationData {
    fn from(signature_verification_data: SignatureVerificationData) -> Self {
        SignatureCreationData {
            email: signature_verification_data.email,
            client_id: signature_verification_data.client_id,
            request_id: signature_verification_data.request_id,
            updated_at: signature_verification_data.updated_at,
            signature_key: signature_verification_data.signature_key,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum SignatureResponse {
    Signature(String),
    VerificationResult(SignatureVerificationResult),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum SignatureVerificationResult {
    Success,
    Fail
}