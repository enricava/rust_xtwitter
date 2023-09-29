use axum::Json;
use serde::Deserialize;
use serde_json::{Value, json};
use crate::{Error, Result};

async fn api_login(payload: Json<LoginPayLoad>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real auth logic.
    if payload.username != "demo" || payload.pwd != "demo_pwd" {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies

    // Create success body.
    let body = Json(json!({
        "result" : {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayLoad {
    username: String,
    pwd: String,
}