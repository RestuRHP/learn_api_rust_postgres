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
    }
}

