use lambda_runtime::{tracing, Context, Error, LambdaEvent};
use crate::signature_request::*;
use crate::signature_request::SignatureVerificationResult::Success;
use crate::signature_request::SignatureVerificationResult::Fail;
use sha2::{Sha256, Digest};
use hex;


pub(crate)async fn function_handler(event: LambdaEvent<SignatureRequest>) -> Result<SignatureResponse, Error> {
    tracing::info!("event: {:?}", &event);

    if let SignatureRequest {
        signature_request_type: SignatureRequestType::SignatureCreationRequest,
        signature_request_payload: SignatureRequestPayload::SignatureCreationRequest(payload)
    } = event.payload {
        return Ok(SignatureResponse::Signature(create_signature(payload)))
    } else if let SignatureRequest {
        signature_request_type: SignatureRequestType::SignatureVerificationRequest,
        signature_request_payload: SignatureRequestPayload::SignatureVerificationRequest(payload)
    } = event.payload {
        return Ok(SignatureResponse::VerificationResult(verify_signature(payload)))
    }
    Err(Error::from("Invalid request"))
}

fn create_signature(signature_creation_data: SignatureCreationData) -> String {
    let email = signature_creation_data.email.clone();
    let client_id = signature_creation_data.client_id.clone();
    let request_id = signature_creation_data.request_id.clone();
    let signature_key = signature_creation_data.signature_key.clone();
    let updated_at = signature_creation_data.updated_at;

    let data = format!("|{}|{}|{}|{}|{}|", email, client_id, request_id, signature_key, updated_at);
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let signature = hex::encode(result);

    signature
}

fn verify_signature(signature_verification_data: SignatureVerificationData) -> SignatureVerificationResult {
    let signature = signature_verification_data.signature_value.clone();
    let creation_data = SignatureCreationData::from(signature_verification_data);
    if create_signature(creation_data) == signature {
        Success
    } else {
        Fail
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use super::*;
    use uuid::Uuid;
    use crate::signature_request::SignatureResponse::{Signature, VerificationResult};

    #[tokio::test]
    async fn test_event_handler() {
        let test_email = "test@example.com".to_string();
        let test_client_id = "client-1".to_string();
        let test_request_id = "request-1".to_string();
        let test_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let test_sign_key = Uuid::new_v4().to_string();

        let creation_request = SignatureRequest {
            signature_request_type: SignatureRequestType::SignatureCreationRequest,
            signature_request_payload: SignatureRequestPayload::SignatureCreationRequest(
                SignatureCreationData{
                    email: test_email.clone(),
                    client_id: test_client_id.clone(),
                    request_id: test_request_id.clone(),
                    updated_at: test_updated,
                    signature_key: test_sign_key.clone()
                }
            )
        };

        let event = LambdaEvent {
            payload:creation_request,
            context: Context::default()
        };

        let creation_response = function_handler(event).await.unwrap();
        let Signature(signature) = creation_response else { todo!()};

        let verification_request = SignatureRequest {
            signature_request_type: SignatureRequestType::SignatureVerificationRequest,
            signature_request_payload: SignatureRequestPayload::SignatureVerificationRequest(
                SignatureVerificationData{
                    email: test_email.clone(),
                    client_id: test_client_id.clone(),
                    request_id: test_request_id.clone(),
                    updated_at: test_updated,
                    signature_key: test_sign_key.clone(),
                    signature_value: signature.to_string()
                }
            )
        };

        let event_2 = LambdaEvent {
            payload: verification_request,
            context: Context::default()
        };

        let verification_response = function_handler(event_2).await.unwrap();
        let VerificationResult(verification_result) = verification_response else { todo!()};
        assert_eq!(Success, verification_result);

        let verification_request_2 = SignatureRequest {
            signature_request_type: SignatureRequestType::SignatureVerificationRequest,
            signature_request_payload: SignatureRequestPayload::SignatureVerificationRequest(
                SignatureVerificationData{
                    email: test_email.clone(),
                    client_id: test_client_id.clone(),
                    request_id: test_request_id.clone(),
                    updated_at: test_updated,
                    signature_key: test_sign_key.clone(),
                    signature_value: Uuid::new_v4().to_string(),
                }
            )
        };

        let event_3 = LambdaEvent {
            payload: verification_request_2,
            context: Context::default()
        };
        let verification_response_2 = function_handler(event_3).await.unwrap();
        let VerificationResult(verification_result_2) = verification_response_2 else { todo!()};
        assert_eq!(Fail, verification_result_2);
    }
}
