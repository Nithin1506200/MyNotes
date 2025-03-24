// @generated automatically by Diesel CLI.

diesel::table! {
    merchant_account (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Varchar,
        address -> Jsonb,
        allowed_payments -> Jsonb,
        active -> Bool,
    }
}
