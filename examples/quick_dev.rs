use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/Henry").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo",
            "pwd": "demopwd"
        }),
    );
    req_login.await?.print().await?;

    let req_create_tweet = hc.do_post(
        "/api/tweets",
        json!({
            "content": "Today is a great day"
        }),
    );
    req_create_tweet.await?.print().await?;

    hc.do_get("/api/tweets").await?.print().await?;

    hc.do_delete("/api/tweets/1").await?.print().await?;
    hc.do_delete("/api/tweets/0").await?.print().await?;

    hc.do_get("/api/tweets").await?.print().await?;

    Ok(())
}
