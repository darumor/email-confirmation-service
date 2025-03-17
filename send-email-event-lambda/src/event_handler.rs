use std::env;
use lambda_runtime::{tracing, Error, LambdaEvent};
use aws_sdk_lambda::{Client};
use aws_smithy_types::Blob;
use aws_lambda_events::event::dynamodb::Event;
use urlencoding::encode;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ses::types::{Destination, Message, Body, Content};

use chrono::{DateTime, Utc};
use serde_json::{json};
use serde_dynamo::{Item, AttributeValue::S, AttributeValue::N, from_item};

use email_confirmation_service_common::email_confirmation_request::{EmailConfirmationRequest, EmailConfirmationServiceApiResponse};
use email_confirmation_service_common::email_confirmation_request::Status::{Pending, Queued};
use email_confirmation_service_common::signature_request::SignatureResponse::Signature;
use email_confirmation_service_common::signature_request::{SignatureRequest, SignatureResponse};

pub(crate)async fn function_handler(event: LambdaEvent<Event>) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;
    println!("{:?}", &payload);
    tracing::info!("Payload: {:?}", payload);

    let records = payload.records;
    for record in records.iter() {
        if record.event_name != "INSERT" && record.event_name != "MODIFY" {
            return Ok(())
        }

        if record.event_source != Some("aws:dynamodb".to_string()) {
            return Ok(())
        }

        let confirmation_request : EmailConfirmationRequest = from_item(record.clone().change.new_image).unwrap();

        if record.event_name == "INSERT" {
            if confirmation_request.status != Queued {
                return Ok(())
            }
            let signature = create_signature(&confirmation_request).await?;
            set_status_to_pending(&confirmation_request, signature).await?;
            return Ok(())
        } else if record.event_name == "MODIFY" {
            if confirmation_request.status != Pending {
                return Ok(())
            }

            let signature = create_signature(&confirmation_request).await?;
            let link_click_handler_service_url = env::var("EMAIL_LINK_CLICK_HANDLER_SERVICE_URL")?;
            let link = format!("{}/confirm?principal={}&signature={}", link_click_handler_service_url, encode(&confirmation_request.pk), signature);

            println!("Created link: {}", &link);
            tracing::info!("Created link: {}", &link);

            let email_message = format_email(confirmation_request.expires_at, link);
            let result = send_email(confirmation_request.email.clone(), email_message).await?;
            result
        }
    }
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

async fn set_status_to_pending(confirmation_request: &EmailConfirmationRequest, signature: String) -> Result<(), Error> {
    let service_url = env::var("EMAIL_CONFIRMATION_REQUEST_SERVICE_URL")?;
    let api_key = env::var("EMAIL_CONFIRMATION_REQUEST_SERVICE_INTERNAL_API_KEY")?;

    let put_url = format!("{}/email-confirmation-requests/{}/status", service_url, encode(&confirmation_request.clone().pk));
    let reqwest_client = reqwest::Client::new();
    let response = reqwest_client
        .put(put_url)
        .header("x-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&json!({"status": "Pending", "signature": signature}))
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

fn format_email(expires_at: u64, link: String) -> String {
    let datetime: DateTime<Utc> = DateTime::<Utc>::from_timestamp(expires_at as i64, 0).unwrap();
    format!("Hi! to confirm your email address, click the link below. The link will expire on {}. \n\n {}. ", datetime, link)
}

async fn send_email(email_address: String, email_message: String) -> Result<(), Error> {
    tracing::info!("Sending email");
    
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = aws_sdk_ses::Client::new(&config);

    let destination = Destination::builder()
        .to_addresses(email_address)
        .build();

    let subject = Content::builder().data("Please, confirm your email.").build()?;
    let body = Body::builder()
        .text(Content::builder().data(email_message).build()?)
        .build();

    let message = Message::builder()
        .subject(subject)
        .body(body)
        .build();

    let email_sender_address = env::var("EMAIL_SENDER_ADDRESS")?;

    // Send the email
    let response = client
        .send_email()
        .source(email_sender_address)  // Change to a verified SES email
        .destination(destination)
        .message(message)
        .send()
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;
    use aws_lambda_events::dynamodb::{EventRecord, StreamRecord};
    use aws_lambda_events::dynamodb::StreamViewType::NewAndOldImages;
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};
    use chrono::{DateTime, TimeZone};

    #[tokio::test]
    async fn test_event_handler() {
        let event = LambdaEvent::new(example_dynamodb_event(), Context::default());
        let response = function_handler(event).await.unwrap();
        assert_eq!((), response);
    }

    #[tokio::test]
    async fn test_another_event_handler() {
        let event = LambdaEvent::new(test_event(), Context::default());
        let response = function_handler(event).await.unwrap();
        assert_eq!((), response);
    }

    #[tokio::test]
    async fn test_another_event_to_json() {
        println!("{}", json!(test_event()));
        //let event = LambdaEvent::new(test_event(), Context::default());
        //let response = function_handler(event).await.unwrap();
        //assert_eq!((), response);
    }

    #[test]
    fn test_example_dynamodb_event() {
        let mut parsed: Event = example_dynamodb_event();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let re_parsed: Event = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, re_parsed);

        let event = parsed.records.pop().unwrap();
        let date = Utc.with_ymd_and_hms(2016, 12, 2,1, 27, 0).unwrap();
        assert_eq!(date, event.change.approximate_creation_date_time);
    }

    fn example_dynamodb_event() -> Event {
        let data = include_bytes!("../fixtures/example-dynamodb-event.json");
        serde_json::from_slice(data).unwrap()
    }

    fn test_event() -> Event {
        Event {
            records: [
                EventRecord {
                    aws_region: "eu-north-1".to_string(),
                    change: StreamRecord {
                        approximate_creation_date_time: DateTime::from_str("2025-03-10T07:41:16Z").unwrap(),
                        keys: Item::from(HashMap::from([("pk".to_string(), S("email@example.com#me_myself_and_i-3#req-3".to_string()))])),
                        new_image: Item::from(HashMap::from(
                            [
                                ("signature_key".to_string(), S("75af2381-ecde-4113-af22-75c2c1407d98".to_string())),
                                ("request_id".to_string(), S("req-3".to_string())),
                                ("status".to_string(), S("Queued".to_string())),
                                ("callback_url".to_string(), S("http://localhost:9000/email-confirmation".to_string())),
                                ("pk".to_string(), S("email@example.com#me_myself_and_i-3#req-3".to_string())),
                                ("expires_at".to_string(), N("1741596076".to_string())),
                                ("client_id".to_string(), S("me_myself_and_i-3".to_string())),
                                ("email".to_string(), S("email@example.com".to_string())),
                                ("updated_at".to_string(), N("1741592476".to_string())),
                                ("created_at".to_string(), N("1741592476".to_string()))
                            ])),
                        old_image: Item::from(HashMap::from([])),
                        sequence_number: Some("14452200000000019503617049".to_string()),
                        size_bytes: 325,
                        stream_view_type: Some(NewAndOldImages)
                    },
                    event_id: "36b242ca5d88a41d00d6df41b8fbc9ff".to_string(),
                    event_name: "INSERT".to_string(),
                    event_source: Some("aws:dynamodb".to_string()),
                    event_version: Some("1.1".to_string()),
                    event_source_arn: Some("arn:aws:dynamodb:us-east-1:123456789012:table/Example-Table/stream/2016-12-01T00:00:00.000".to_string()),
                    user_identity: None,
                    record_format: None,
                    table_name: None
                }
            ].to_vec()
        }
    }
}
