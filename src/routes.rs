mod api;
mod frontend;

use std::collections::HashMap;
use std::sync::Arc;
use poem::{EndpointExt, Route};
use crate::config::Item;
use crate::middleware::BasicAuth;
use crate::routes::api::create_transaction;
use crate::routes::frontend::index;

pub(crate) fn all_routes(config: Arc<HashMap<String, Vec<Item>>>, username: String, password: String) -> impl poem::Endpoint {
    let inner = Route::new()
        .at("/", index)
        .at("/api/create/:id", create_transaction)
        .data(config);

    BasicAuth { inner, username, password }
}
