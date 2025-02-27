use std::env;
use reqwest::Client;
use lambda_http::{Body, Error, Request, RequestExt, RequestPayloadExt, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request

    let method = event.method().as_str();
    let url = event.uri().to_string();
    let path_params = event.path_parameters();
    let query_params = event.query_string_parameters();
    let path = event.raw_http_path();

    let response_as_string = call_target_lambda().await?;

    let request = format!(
        "method: {}\nurl: {}\n path_params: {:?}\nquery_params: {:?}\npath: {}\nevent: {:?}\nresponse: {}",
        method, url, path_params, query_params, path, event, response_as_string);

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(request.into())
        .map_err(Box::new)?;

    Ok(resp)
}


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
