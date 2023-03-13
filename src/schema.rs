// @generated automatically by Diesel CLI.

diesel::table! {
    photos (id) {
        id -> Uuid,
        user_id -> Uuid,
        url -> Nullable<Varchar>,
    }
}

diesel::table! {
    tweets (id) {
        id -> Uuid,
        message -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
    }
}

diesel::joinable!(photos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    photos,
    tweets,
    users,
);
