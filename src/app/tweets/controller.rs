use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::app::{
    db::DbPool,
    tweets::{
        models::TweetPayload,
        repository::{add_tweet, delete_tweet, find_all, find_by_id, update_tweet},
    },
};

#[get("/tweets")]
pub async fn tweets_index(db_pool: web::Data<DbPool>) -> impl Responder {
    let tweets_result = web::block(move || {
        let connection = &mut db_pool.get().unwrap();
        find_all(connection)
    })
    .await;

    if let Err(error) = tweets_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", error));
    }

    match tweets_result.unwrap() {
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        Ok(tweets_result) => HttpResponse::Ok().json(tweets_result),
    }
}

#[get("/tweets/{id}")]
pub async fn tweet_index(tweet_id: web::Path<Uuid>, db_pool: web::Data<DbPool>) -> impl Responder {
    let tweets_result = web::block(move || {
        let connection = &mut db_pool.get().unwrap();
        find_by_id(&tweet_id.into_inner(), connection)
    })
    .await;

    if let Err(error) = tweets_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", error));
    }

    match tweets_result.unwrap() {
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        Ok(tweets_result) => HttpResponse::Ok().json(tweets_result),
    }
}

#[post("/tweets")]
pub async fn create(
    db_pool: web::Data<DbPool>,
    payload: web::Json<TweetPayload>,
) -> impl Responder {
    let tweet_result = web::block(move || {
        let connection = &mut db_pool.get().unwrap();
        add_tweet(&payload.message, connection)
    })
    .await;

    if let Err(error) = tweet_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", error));
    }

    match tweet_result.unwrap() {
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        Ok(result) => return HttpResponse::Ok().json(result),
    }
}

#[put("/tweets/{id}")]
pub async fn update(
    tweet_id: web::Path<Uuid>,
    payload: web::Json<TweetPayload>,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    let tweet_result = web::block(move || {
        let connection = &mut db_pool.get().unwrap();
        update_tweet(&tweet_id.into_inner(), &payload.message, connection)
    })
    .await;

    if let Err(error) = tweet_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", error));
    }

    match tweet_result.unwrap() {
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        Ok(result) => return HttpResponse::Ok().json(result),
    }
}

#[delete("/tweets/{id}")]
pub async fn delete(tweet_id: web::Path<Uuid>, db_pool: web::Data<DbPool>) -> impl Responder {
    let tweet_result = web::block(move || {
        let connection = &mut db_pool.get().unwrap();
        delete_tweet(&tweet_id.into_inner(), connection)
    })
    .await;

    if let Err(error) = tweet_result {
        return HttpResponse::InternalServerError().body(format!("{:?}", error));
    }

    match tweet_result.unwrap() {
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        Ok(_) => return HttpResponse::Ok().finish(),
    }
}
