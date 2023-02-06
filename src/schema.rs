// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Varchar,
        description -> Text,
        created_at -> Timestamptz,
        notification_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        is_completed -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        image_url -> Nullable<Varchar>,
    }
}

diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
