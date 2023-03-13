use crate::app::db::DbError;
use diesel::prelude::*;
use uuid::Uuid;

use super::models::{NewTweet, Tweet};
use crate::schema::tweets::dsl::*;

pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Tweet>, DbError> {
    let tweet = tweets.load::<Tweet>(conn)?;
    Ok(tweet)
}

pub fn find_by_id(tweet_id: &Uuid, conn: &mut PgConnection) -> Result<Option<Tweet>, DbError> {
    let tweet = tweets
        .filter(id.eq(tweet_id))
        .first::<Tweet>(conn)
        .optional()?;

    Ok(tweet)
}

pub fn add_tweet(_message: &str, conn: &mut PgConnection) -> Result<Tweet, DbError> {
    let new_tweet = NewTweet {
        created_at: chrono::Local::now().naive_local(),
        message: _message,
    };

    let res = diesel::insert_into(tweets)
        .values(&new_tweet)
        .get_result(conn)?;

    Ok(res)
}

pub fn update_tweet(
    tweet_id: &Uuid,
    tweet_message: &String,
    conn: &mut PgConnection,
) -> Result<Tweet, DbError> {
    let res = diesel::update(tweets.find(tweet_id))
        .set(message.eq(tweet_message))
        .get_result(conn)?;

    Ok(res)
}

pub fn delete_tweet(tweet_id: &Uuid, conn: &mut PgConnection) -> Result<Tweet, DbError> {
    let res = diesel::delete(tweets.find(tweet_id)).get_result(conn)?;

    Ok(res)
}
