use std::env;
use reqwest::Client;
use urlencoding::encode;
use lambda_http::{Body, Error, Request, RequestExt, RequestPayloadExt, Response};
use lambda_runtime::{tracing};
use crate::email_confirmation_request::{SanitizedEmailConfirmationRequest, Status, EmailConfirmationServiceApiResponse};

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






    let raw_data = get_raw_data(
        service_url, api_key, principal.to_string(), signature.to_string()).await.unwrap();

    tracing::info!("raw data: {:?}", &raw_data);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("{:?}", raw_data).into())
        .map_err(Box::new)?;

    Ok(resp)

        /*
    let confirmation_request = get_confirmation_request_by_principal(
        service_url, api_key, principal.to_string(), signature.to_string()).await?;

    if !expiration_date_is_valid(&confirmation_request) {
        return Err(Error::from(format!("Request expired. Expiration date: {}", &confirmation_request.expires_at)));
    }

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("{:?}", confirmation_request).into())
        .map_err(Box::new)?;

    Ok(resp)
    */
}


async fn get_raw_data(service_url: String, api_key: String, principal: String, signature: String) -> Result<String, Error> {
    let get_one_url = format!("{}/email-confirmation-requests/{}?signature={}", service_url, encode(&principal), encode(&signature));

    let reqwest_client = Client::new();
    let response = reqwest_client
        .get(&get_one_url)
        //.header("x-api-key", api_key)
        .header("Content-Type", "application/json")
        //.json(&json!({"name": "Alice"}))
        .send()
        .await
        .unwrap();

    tracing::info!("url: {:?}", &get_one_url);

    let text= response.text().await.unwrap();

    tracing::info!("response: {:?}", &text);

    Ok(text)
}


async fn get_confirmation_request_by_principal(service_url: String, api_key: String, principal: String, signature: String) -> Result<SanitizedEmailConfirmationRequest, Error> {
    let get_one_url = format!("{}/email-confirmation-requests/{}?signature={}", service_url, principal, signature);

    let reqwest_client = Client::new();
    let response = reqwest_client
        .get(get_one_url)
        .header("x-api-key", api_key)
        .header("Content-Type", "application/json")
        //.json(&json!({"name": "Alice"}))
        .send()
        .await
        .unwrap();

    let json_data : EmailConfirmationServiceApiResponse = response.json().await?;
    if json_data.error {
        return Err(Error::from(format!("Invalid response: {:?}", json_data)));
    }

    Ok(json_data.request)

  /*  Ok(SanitizedEmailConfirmationRequest {
        request_id: String::from("asd"),
        status: Status::Pending,
        expires_at: 5,
        updated_at: 4,
        pk: String::from("asd"),
        email: String::from("jarkko@example.com"),
        client_id: String::from("asd"),
        created_at: 0,
        callback_url: String::from("https://example.com"),
    })
    */

}

fn expiration_date_is_valid(confirmation_request: &SanitizedEmailConfirmationRequest) -> bool {
    true
}


// let response_as_string = call_target_lambda().await?;
async fn call_target_lambda() -> Result<String, Error> {
    let service_url = env::var("EMAIL_CONFIRMATION_REQUEST_SERVICE_URL")?;
    let api_key = env::var("EMAIL_CONFIRMATION_REQUEST_SERVICE_INTERNAL_API_KEY")?;

    let pk = "";
    let get_all_url = format!("{}/email-confirmation-requests", service_url);
    let get_one_url = format!("{}/email-confirmation-requests/{}", service_url, pk);
    let put_url = format!("{}/email-confirmation-requests/{}/status", service_url, pk);

    make_the_get_call(get_all_url).await

}

async fn make_the_get_call(get_all_url: String) -> Result<String, Error> {
    let reqwest_client = Client::new();
    let response = reqwest_client
        .get(get_all_url)
        //.header("x-api-key", "your-api-key-here")
        .header("Content-Type", "application/json")
        //.json(&json!({"name": "Alice"}))
        .send()
        .await
        .unwrap();

    Ok(response.text().await.unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use lambda_http::{Request, RequestExt};

    #[tokio::test]
    async fn test_generic_http_handler() {
        let request = Request::default();

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello world, this is an AWS Lambda HTTP request"
        );
    }

    #[tokio::test]
    async fn test_http_handler_with_query_string() {
        let mut query_string_parameters: HashMap<String, String> = HashMap::new();
        query_string_parameters.insert("name".into(), "handle-email-link-click-endpoint".into());

        let request = Request::default()
            .with_query_string_parameters(query_string_parameters);

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello handle-email-link-click-endpoint, this is an AWS Lambda HTTP request"
        );
    }
}
