use crate::app::tweets::controller::{create, delete, tweet_index, tweets_index, update};
use actix_web::web::ServiceConfig;

pub fn register_tweets(app: &mut ServiceConfig) {
    app.service(create)
        .service(update)
        .service(delete)
        .service(tweet_index)
        .service(tweets_index);
}
