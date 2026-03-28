use axum::{Json, Router, routing::get};
use serde_json::json;

use crate::state::App;

pub fn get_router() -> Router<App> {
    // https://javascript.info/fetch-crossorigin
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
    // let cors = CorsLayer::new()
    //     // TODO: Change this to the actual origin
    //     .allow_origin(Any)
    //     .allow_methods([
    //         Method::GET,
    //         Method::POST,
    //         Method::DELETE,
    //         Method::OPTIONS,
    //         Method::PUT,
    //         Method::PATCH,
    //     ])
    //     .allow_headers([
    //         header::CONTENT_TYPE,
    //         header::ORIGIN,
    //         header::AUTHORIZATION,
    //         header::CACHE_CONTROL,
    //     ])
    //     .max_age(Duration::from_secs(3600));

    Router::new().route("/", get(api_root)) // .layer(cors)
}

async fn api_root() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
