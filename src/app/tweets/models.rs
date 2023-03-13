use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::diesel::prelude::*;
use crate::schema::tweets;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Tweet {
    pub id: Uuid,
    pub message: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = tweets)]
pub struct NewTweet<'a> {
    pub message: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetPayload {
    pub message: String,
}
