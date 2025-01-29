
use aws_sdk_dynamodb::Client;

#[derive(Clone, Debug)]
pub struct EmailConfirmationRequestService {
    //db_client: Client,
    table_name: String,
}

impl EmailConfirmationRequestService {
    pub fn new() -> Self {
        Self {

            table_name: String::from("fake")
        }
    }
    /*pub fn new_(db_client: Client, table_name: &str) -> Self {
        Self {
            db_client,
            table_name: table_name.to_owned(),
        }
    }*/
}