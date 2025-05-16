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

    let principal  = query_params.first("principal").unwrap();
    let signature = query_params.first("signature").unwrap();
    let service_url = env::var("EMAIL_CONFIRMATION_REQUEST_SERVICE_URL")?;
    let api_key = env::var("EMAIL_CONFIRMATION_REQUEST_SERVICE_INTERNAL_API_KEY")?;
    let self_service_url = env::var("EMAIL_LINK_CLICK_HANDLER_SERVICE_URL")?;

    let confirmation_request = get_confirmation_request_by_principal(
        &service_url, &api_key, principal.to_string(), signature.to_string()).await?;

    if !expiration_date_is_valid(&confirmation_request) {
        return get_expired_response().await;
    }

    if method == "GET" {
        return get_confirm_button_response(&self_service_url, &confirmation_request.email, &principal, &signature).await;
    }

    if method == "POST" {
        let updated_request = set_request_status_as_confirmed(&service_url, &api_key, confirmation_request.pk, signature.to_string()).await?;
        return get_confirmed_response(updated_request).await;
    }

    Err(Error::from(format!("Invalid method: {}", method)))
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

async fn get_expired_response() -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("<html><head>
            <title>Confirmation request expired</title>
        </head><body>
            <h1>Confirmation request expired.</h1>
            <p>Please, re-request confirmation.</p>
        </body><html>".into())
        .map_err(Box::new)?;
    Ok(resp)
}

async fn get_confirm_button_response(self_service_url: &str, email: &str, principal: &str, signature: &str) -> Result<Response<Body>, Error> {
    let action_url = format!("{}/confirm?principal={}&signature={}", self_service_url, encode(principal), signature);
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(
            format!("<html><head>
                <title>Confirm email address</title>
            </head><body>
                <h1>Confirm email address</h1>
                <p>Confirm your email address '{}' by clicking the button below.</p>
                <form method=\"POST\" action=\"{}\">
                    <input type=\"submit\" value=\"Confirm\" />
                </form>
            </body><html>", email, action_url.as_str()).into())
        .map_err(Box::new)?;
    Ok(resp)
}

async fn get_confirmed_response(updated_request: SanitizedEmailConfirmationRequest) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("<html><head>
            <title>Email address confirmed</title>
        </head><body>
            <h1>Email address confirmed</h1>
            <p>Your email address '{}' is confirmed.</p>
        </body><html>", updated_request.email).into())
        .map_err(Box::new)?;
    Ok(resp)
}


#[cfg(test)]
mod tests {
    /*
    use super::*;
    #[tokio::test]
    async fn test_get_email_confirmation_button_html() {
        let principal = "foobar@example.com";
        let signature = "foobar";
        let html =  get_email_confirmation_button_html(principal, signature);
        println!("{:?}", &html);

        assert_eq!(html, "
        <html><head>
            <title>Confirm email address</title>
        </head><body>
            <h1>Confirm email address</h1>
            <p>Confirm your email address 'foobar@example.com' by clicking the button below.</p>
            <form method=\"POST\" action=\"/confirm?principal=foobar%40example.com&signature=foobar\">
                <input type=\"submit\" value=\"Confirm\" />
            </form>
        </body><html>
    ");
    }*/
}