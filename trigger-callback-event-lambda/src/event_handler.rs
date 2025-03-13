use std::env;
use lambda_runtime::{tracing, Error, LambdaEvent};
use aws_lambda_events::event::dynamodb::Event;
use aws_sdk_lambda::Client;
use aws_smithy_types::Blob;
use chrono::{Utc};
use urlencoding::encode;

use serde_json::{json};
use serde_dynamo::{Item, AttributeValue::S, AttributeValue::N, from_item};

use email_confirmation_service_common::email_confirmation_request::{EmailConfirmationRequest, EmailConfirmationServiceApiResponse, Status};
use email_confirmation_service_common::email_confirmation_request::Status::{Confirmed};
use email_confirmation_service_common::signature_request::SignatureResponse::Signature;
use email_confirmation_service_common::signature_request::{SignatureRequest, SignatureResponse};
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct ConfirmationMessage {
    email: String,
    status: Status,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct ConfirmationMessageResponse {
    message: String,
}

pub(crate)async fn function_handler(event: LambdaEvent<Event>) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;
    tracing::info!("Payload: {:?}", payload);

    let records = payload.records;
    for record in records.iter() {
        if record.event_name != "MODIFY" {
            return Ok(())
        }

        if record.event_source != Some("aws:dynamodb".to_string()) {
            return Ok(())
        }

        let confirmation_request: EmailConfirmationRequest = from_item(record.clone().change.new_image).unwrap();
        if confirmation_request.status != Confirmed {
            return Ok(())
        }

        let result = trigger_callback(confirmation_request.clone()).await;
        match result {
            Ok(_) => {
                let signature = create_signature(&confirmation_request).await?;
                tracing::info!("Setting status to done for {}.", &confirmation_request.pk);
                set_status_to_done(&confirmation_request, signature).await?;
            },
            Err(_) => {
                tracing::error!("Callback failed for {}.", &confirmation_request.pk);
            }
        }
    }
    Ok(())
}

async fn trigger_callback(email_confirmation_request: EmailConfirmationRequest) -> Result<(), Error> {
    let callback_url = email_confirmation_request.callback_url;
    let message_json = json!(ConfirmationMessage{email: email_confirmation_request.email.clone(), status: Confirmed});
    let reqwest_client = reqwest::Client::new();
    let response = reqwest_client
        .post(callback_url)
        .header("Content-Type", "application/json")
        .json(&message_json)
        .send()
        .await
        .unwrap();
    Ok(())
}

async fn create_signature(email_confirmation_request: &EmailConfirmationRequest) -> Result<String, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let payload = json!(SignatureRequest::signature_creation_request(email_confirmation_request.clone()));
    let function_name = env::var("SIGNATURE_SERVICE_LAMBDA_FUNCTION_NAME");

    if let Ok(function_name) = function_name {
        let response = client.invoke()
            .function_name(function_name)
            .payload(Blob::new(payload.to_string()))
            .send()
            .await
            .unwrap();

        if let Some(payload) = response.payload {
            let response_str = String::from_utf8(payload.into_inner()).expect("Invalid UTF-8");
            let result: SignatureResponse = serde_json::from_str(&response_str).expect("Invalid JSON");

            if let Signature(signature) = result {
                return Ok(signature);
            }
        }
    }
    Err(Error::from("Error creating signature"))
}

async fn set_status_to_done(confirmation_request: &EmailConfirmationRequest, signature: String) -> Result<(), Error> {
    let service_url = env::var("EMAIL_CONFIRMATION_REQUEST_SERVICE_URL")?;
    let api_key = env::var("EMAIL_CONFIRMATION_REQUEST_SERVICE_INTERNAL_API_KEY")?;

    let put_url = format!("{}/email-confirmation-requests/{}/status", service_url, encode(&confirmation_request.clone().pk));
    let reqwest_client = reqwest::Client::new();
    let response = reqwest_client
        .put(put_url)
        .header("x-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&json!({"status": "Done", "signature": signature}))
        .send()
        .await
        .unwrap();

    let json_data : EmailConfirmationServiceApiResponse = response.json().await?;
    if json_data.error {
        Err(Error::from("Email confirmation service error"))
    } else {
        Ok(())
    }
}
#[cfg(test)]
mod tests {

}
