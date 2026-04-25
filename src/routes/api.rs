use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use anyhow::Context;
use poem::{handler, IntoResponse, Response};
use poem::web::{Data, Path};
use anyhow::Result;
use poem::error::BadRequest;
use poem::http::StatusCode;
use reqwest::Client;
use serde_json::json;
use time::macros::format_description;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::config::{find_by_id, Item};

#[handler]
pub(crate) async fn create_transaction(
    Path(id): Path<String>,
    Data(config): Data<&Arc<HashMap<String, Vec<Item>>>>,
) -> Result<impl IntoResponse> {
    let token = std::env::var("LUNCHMONEY_API_KEY")
        .context("could not find env variable `LUNCHMONEY_API_KEY`")?;

    let uuid = Uuid::from_str(&id).map_err(|err| BadRequest(err))?;
    let item = match find_by_id(uuid, config) {
        None => return Ok(Response::builder().status(StatusCode::NOT_FOUND).body("item not found")),
        Some(item) => item
    };

    let now = OffsetDateTime::now_utc().format(format_description!("[year]-[month]-[day]"))
        .context("failed to format date")?;

    let body = json!({
        "date": now,
        "amount": item.amount,
        "currency": item.currency,
        "payee": item.payee,
        "category_id": item.category_id,
        "manual_account_id": item.account_id,
        "status": "unreviewed",
    });

    let response = Client::new().post("https://api.lunchmoney.dev/v2/transactions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .json(&json!({
            "transactions": [body],
            "apply_rules": true,
        }))
        .send()
        .await
        .context("failed to send request to lunchmoney")?;

    let status = response.status();
    let text = response.text().await.context("failed to read lunchmoney response body")?;

    if status.is_success() {
        Ok(Response::builder().status(StatusCode::OK).body("transaction created"))
    } else {
        Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(text))
    }
}
