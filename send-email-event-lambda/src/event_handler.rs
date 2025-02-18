use lambda_runtime::{tracing, Error, LambdaEvent};
use aws_lambda_events::event::dynamodb::Event;
use chrono::{Utc};


/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
pub(crate)async fn function_handler(event: LambdaEvent<Event>) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;
    println!("{:?}", &payload);
    tracing::info!("Payload: {:?}", payload);

    /*
     const data = event.Records[0];

   if(data.eventName != 'INSERT' && data.eventSource != 'aws:dynamodb'){

       return;

   }
    */

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};
    use chrono::TimeZone;

    #[tokio::test]
    async fn test_event_handler() {
        let event = LambdaEvent::new(example_dynamodb_event(), Context::default());
        let response = function_handler(event).await.unwrap();
        assert_eq!((), response);
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
}
