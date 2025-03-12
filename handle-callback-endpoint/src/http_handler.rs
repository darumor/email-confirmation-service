use lambda_http::{tracing, Body, Error, Request, Response};

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let method = event.method().as_str();
    if method != "POST" {
        return Err(Error::from(format!("Invalid method: {}. Callback request should use POST.", method)));
    }

    let body_string = String::from_utf8_lossy(event.body().as_ref()).into_owned();
    if !body_string.is_empty() {
        tracing::info!("{:?}", &body_string);
        respond(body_string)
    } else {
        tracing::info!("No body in request");
        respond(" -- No body found in request -- ".to_owned())
    }
}

fn respond(body: String) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(format!("{{ \"message\":\"Confirmation for '{}' received.\"", body).into())
        .map_err(Box::new)?;
    Ok(resp)
}