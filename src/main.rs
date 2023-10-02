pub use self::error::{Error, Result};

use crate::model::ModelController;
use std::net::SocketAddr;
use axum::{Router, response::{Html, IntoResponse, Response}, routing::{get, get_service}, extract::{Query, Path}, middleware};
use serde::Deserialize;
use tower_http::services::ServeDir;
use tower_cookies::CookieManagerLayer;

mod ctx;
mod error;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController
    let mc = ModelController::new().await?;
    
    let routes_apis = web::routes_tweets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    // Layers run bottom to top
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:      --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on http://{addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion:   --- Start Server

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response{
    println!("->> {:<12} - main_response_mapper {}", "RES_MAPPER", "\n");
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:          --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// /hello?name=Henry
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("Hello {name}!"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello {name}!"))
}
// endregion:       --- Routes Hello
