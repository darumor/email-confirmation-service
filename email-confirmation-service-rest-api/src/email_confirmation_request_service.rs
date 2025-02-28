use std::string::ToString;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{bail, Ok, Result};
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use axum::Json;
use serde_dynamo::{from_item, from_items, to_item};
use serde_json::{json, Value};
use crate::email_confirmation_request::{EmailConfirmationRequest, SanitizedEmailConfirmationRequest, Status};
use crate::handler_params::QueryParams;

pub const NOT_FOUND_ERROR:&str = "Request does not exist for pk";

#[derive(Clone, Debug)]
pub struct EmailConfirmationRequestService {
    db_client: Client,
    table_name: String,
}

impl EmailConfirmationRequestService {
    pub fn new(db_client: Client, table_name: &str) -> Self {
        Self {
            db_client,
            table_name: table_name.to_owned(),
        }
    }

    pub async fn get_email_confirmation_requests(&self, params: QueryParams) -> Result<Json<Value>> {
       let pk_result = EmailConfirmationRequest::pk_from_query_params(&params);
        if let Result::Ok(id_string) = pk_result {
            return self.get_email_confirmation_request_single(id_string).await;
        }

        let builder = self.db_client.scan().table_name(&self.table_name);
        let mut results = builder.clone().send().await?;

        if let Some(items) = results.items {
            let mut requests: Vec<SanitizedEmailConfirmationRequest> = from_items(items)?;
            while let Some(last_evaluated_key) = &results.last_evaluated_key {
                results = builder
                    .clone()
                    .set_exclusive_start_key(Some(last_evaluated_key.to_owned()))
                    .send()
                    .await?;
                if let Some(new_items) = results.items {
                    let mut new_requests: Vec<SanitizedEmailConfirmationRequest> = from_items(new_items)?;
                    requests.append(&mut new_requests);
                } else {
                    break;
                }
            }

            Ok(Json(json!({
                    "error": false,
                    "requests": requests
                })))
        } else  {
            Ok(Json(json!({})))
        }
    }

    async fn request_exist(&self, pk: &str) -> Result<bool> {
        let results = self
            .db_client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("#name = :value")
            .expression_attribute_names("#name", "pk")
            .expression_attribute_values(":value", AttributeValue::S(pk.to_owned()))
            .send()
            .await?;

        Ok(results.count > 0)
    }

    pub async fn post_email_confirmation_request(&self, ec_request: EmailConfirmationRequest, signature: String) -> Result<Json<Value>> {
        if self.request_exist(&ec_request.pk).await? {
            bail!("Request exists!")
        }
        let item = to_item(ec_request)?;

       let builder = self
            .db_client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item));

        builder.send().await?;

        Ok(Json(json!({
           "error": false,
            "message": format!("Request added. Signature: {}", signature),
        })))
    }

    pub async fn get_email_confirmation_request_single(&self, pk: String) -> Result<Json<Value>> {
            let results = self
                .db_client
                .query()
                .table_name(&self.table_name)
                .key_condition_expression("#name = :value")
                .expression_attribute_names("#name", "pk")
                .expression_attribute_values(":value", AttributeValue::S(pk.to_owned()))
                .send()
                .await?;
            if results.count == 0
                || results.items.is_none()
                || results.items.clone().unwrap().is_empty()
            {
                bail!("{NOT_FOUND_ERROR}: {pk}!")
            }

            let item = results.items.unwrap().first().unwrap().to_owned();
            let request: SanitizedEmailConfirmationRequest = from_item(item)?;

            Ok(Json(json!({
                "error": false,
                "request": request
            })))
    }

    pub(crate) async fn get_email_confirmation_request_internal(&self, pk: String) -> Result<EmailConfirmationRequest> {
        let results = self
            .db_client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("#name = :value")
            .expression_attribute_names("#name", "pk")
            .expression_attribute_values(":value", AttributeValue::S(pk.to_owned()))
            .send()
            .await?;
        if results.count == 0
            || results.items.is_none()
            || results.items.clone().unwrap().is_empty()
        {
            bail!("{NOT_FOUND_ERROR}: {pk}!")
        }

        let item = results.items.unwrap().first().unwrap().to_owned();
        let confirmation_request: EmailConfirmationRequest = from_item(item)?;

        Ok(confirmation_request)
    }

    pub async fn delete_email_confirmation_request_single(&self, pk: String) -> Result<Json<Value>> {
        if !self.request_exist(&pk).await? {
            bail!("{NOT_FOUND_ERROR}: {pk}!")
        }

        self.db_client
            .delete_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(pk.clone()))
            .send()
            .await?;

        Ok(Json(json!({
            "error": false,
           "message": "Request for pk: ".to_owned() + &pk + " deleted."
        })))
    }

    pub async fn put_email_confirmation_request_status(&self, pk: String, status: Status) -> Result<EmailConfirmationRequest> {
        if !self.request_exist(&pk).await? {
            bail!("{NOT_FOUND_ERROR}: {pk}!")
        }

        let updated_at = format!("{}", SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
        self.db_client
            .update_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(pk.clone()))

            .update_expression("set #name1 = :value1, #name2 = :value2")
            .expression_attribute_names("#name1", "status")
            .expression_attribute_names("#name2", "updated_at")
            .expression_attribute_values(":value1", AttributeValue::S(status.to_string()))
            .expression_attribute_values(":value2", AttributeValue::N(updated_at))

            .send()
            .await?;

        Ok(self.get_email_confirmation_request_internal(pk).await?)
    }
}