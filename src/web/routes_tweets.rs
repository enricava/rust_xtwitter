use crate::ctx::Ctx;
use crate::model::{ModelController, Tweet, TweetForCreate};
use crate::Result;
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tweets", post(create_tweet).get(list_tweets))
        .route("/tweets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_tweet(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(tweet_fc): Json<TweetForCreate>,
) -> Result<Json<Tweet>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let tweet = mc.create_tweet(ctx, tweet_fc).await?;

    Ok(Json(tweet))
}

async fn list_tweets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Tweet>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tweets = mc.list_tickets(ctx).await?;

    Ok(Json(tweets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Tweet>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let tweet = mc.delete_ticket(ctx, id).await?;

    Ok(Json(tweet))
}
