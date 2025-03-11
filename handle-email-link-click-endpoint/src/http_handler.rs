use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::Client;
use urlencoding::encode;
use lambda_http::{Body, Error, Request, RequestExt, Response};
use serde_json::json;
use email_confirmation_service_common::email_confirmation_request::{SanitizedEmailConfirmationRequest, EmailConfirmationServiceApiResponse};

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let path = event.raw_http_path();
    let method = event.method().as_str();
    let query_params = event.query_string_parameters();

    if path != "/confirm" {
        return Err(Error::from(format!("Invalid path: {}", path)));
    }

    if method != "GET" {
        return Err(Error::from(format!("Invalid method: {}", method)));
    }

    let principal  = query_params.first("principal").unwrap();
    let signature = query_params.first("signature").unwrap();
    let service_url = env::var("EMAIL_CONFIRMATION_REQUEST_SERVICE_URL")?;
    let api_key = env::var("EMAIL_CONFIRMATION_REQUEST_SERVICE_INTERNAL_API_KEY")?;

    let confirmation_request = get_confirmation_request_by_principal(
        &service_url, &api_key, principal.to_string(), signature.to_string()).await?;

    if !expiration_date_is_valid(&confirmation_request) {
        let msg = "Email confirmation request expired. Please, re-request confirmation.";
        let resp = Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body(format!("{:?}", msg).into())
            .map_err(Box::new)?;
        return Ok(resp)
    }

    let updated_request = set_request_status_as_confirmed(&service_url, &api_key, confirmation_request.pk, signature.to_string()).await?;
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("Email address {} confirmed.", updated_request.email).into())
        .map_err(Box::new)?;

    Ok(resp)
}

async fn set_request_status_as_confirmed(service_url: &str, api_key: &str, principal: String, signature: String) -> Result<SanitizedEmailConfirmationRequest, Error> {
    let put_url = format!("{}/email-confirmation-requests/{}/status", service_url, encode(&principal));
    let reqwest_client = Client::new();
    let response = reqwest_client
        .put(put_url)
        .header("x-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&json!({"status": "Confirmed", "signature": signature}))
        .send()
        .await
        .unwrap();

    let json_data : EmailConfirmationServiceApiResponse = response.json().await?;
    Ok(json_data.request)
}

async fn get_confirmation_request_by_principal(service_url: &str, api_key: &str, principal: String, signature: String) -> Result<SanitizedEmailConfirmationRequest, Error> {
    let get_one_url = format!("{}/email-confirmation-requests/{}?signature={}", service_url, encode(&principal), signature);
    let reqwest_client = Client::new();
    let response = reqwest_client
        .get(get_one_url)
        .header("x-api-key", api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    let json_data : EmailConfirmationServiceApiResponse = response.json().await?;
    Ok(json_data.request)
}

fn expiration_date_is_valid(confirmation_request: &SanitizedEmailConfirmationRequest) -> bool {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    now < confirmation_request.expires_at
}
