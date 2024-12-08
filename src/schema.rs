// @generated automatically by Diesel CLI.

diesel::table! {
    tb_user (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Nullable<Varchar>,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        #[max_length = 100]
        address -> Nullable<Varchar>,
        create_at -> Nullable<Timestamptz>,
        update_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::table! {
    user_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Text,
        created_at -> Nullable<Timestamptz>,
        expired_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(user_tokens -> tb_user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tb_user,
    user_tokens,
);
