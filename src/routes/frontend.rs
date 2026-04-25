use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use poem::{handler, IntoResponse, Response};
use poem::http::StatusCode;
use poem::web::Data;
use crate::config::Item;

#[handler]
pub(crate) async fn index(
    Data(config): Data<&Arc<HashMap<String, Vec<Item>>>>,
) -> Result<impl IntoResponse> {
    let mut groups: Vec<(&String, &Vec<Item>)> = config.iter().collect();
    groups.sort_by_key(|(k, _)| k.as_str());

    let mut body_html = String::new();
    for (group_name, items) in groups {
        let mut items_html = String::new();
        for item in items {
            items_html.push_str(&format!(
                r#"<button class="item" data-id="{id}" onclick="createTransaction(this)">
                    <span class="icon">{icon}</span>
                    <span class="name">{name}</span>
                </button>"#,
                id = item.id,
                icon = item.icon,
                name = item.name,
            ));
        }
        body_html.push_str(&format!(
            r#"<section>
                <h2>{group}</h2>
                <div class="items">{items}</div>
            </section>"#,
            group = group_name,
            items = items_html,
        ));
    }

    let html = include_str!("index.html").replace("{body}", &body_html);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(html))
}
