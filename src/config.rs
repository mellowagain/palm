use std::collections::HashMap;
use std::fs;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Item {
    pub(crate) icon: String,
    pub(crate) name: String,
    pub(crate) category_id: i32,
    pub(crate) currency: String,
    pub(crate) amount: f32,
    pub(crate) account_id: i32,
}

pub(crate) fn load_config() -> Result<HashMap<String, Vec<Item>>> {
    let content = fs::read_to_string("./config.json").context("failed to read config file")?;
    let map = serde_json::from_str(&content).context("failed to deserialize config file json")?;

    Ok(map)
}
