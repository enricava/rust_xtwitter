// Simplistic Model Layer with mock-store

use crate::{Error, Result, ctx::Ctx};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region:          --- Tweet Types
#[derive(Clone, Debug, Serialize)]
pub struct Tweet {
    pub id: u64,
    pub cid: u64,   // creator uid
    pub content: String,

}

#[derive(Deserialize)]
pub struct TweetForCreate {
    pub content: String,
}
// endregion:       --- Tweet Types

// region:          --- Model Controller

//Clone does not clone the vector, only the arc
#[derive(Clone)]
pub struct ModelController {
    tweets_store: Arc<Mutex<Vec<Option<Tweet>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tweets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_tweet(
        &self,
        ctx: Ctx,
        tweet_fc: TweetForCreate
    ) -> Result<Tweet> {
        let mut store = self.tweets_store.lock().unwrap();

        let id = store.len() as u64;
        let tweet = Tweet {
            id,
            cid: ctx.user_id(),
            content: tweet_fc.content,
        };
        store.push(Some(tweet.clone()));

        Ok(tweet)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Tweet>> {
        let store = self.tweets_store.lock().unwrap();

        let tweets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tweets)
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Tweet> {
        let mut store = self.tweets_store.lock().unwrap();

        let tweet = store.get_mut(id as usize).and_then(|t| t.take());

        tweet.ok_or(Error::TweetDeleteFailIdNotFound { id: id })
    }
}
// endregion:       --- Model Controller
