use std::collections::HashMap;
use tokio::fs;
use anyhow::{Context, Result};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub(crate) struct Item {
    #[serde(skip_deserializing, default = "Uuid::new_v4")]
    pub(crate) id: Uuid,

    pub(crate) icon: String,
    pub(crate) name: String,
    pub(crate) category_id: i32,
    pub(crate) currency: String,
    pub(crate) amount: String,
    pub(crate) payee: String,
    pub(crate) account_id: i32,
}

pub(crate) async fn load_config() -> Result<HashMap<String, Vec<Item>>> {
    let content = fs::read_to_string("./config.json").await.context("failed to read config file")?;
    let map = serde_json::from_str(&content).context("failed to deserialize config file json")?;

    Ok(map)
}

pub(crate) fn find_by_id(uuid: Uuid, map: &HashMap<String, Vec<Item>>) -> Option<&Item> {
    map.values().flat_map(|items| items.iter()).find(|&item| item.id == uuid)
}
