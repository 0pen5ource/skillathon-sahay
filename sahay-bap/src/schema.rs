// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        telegram_handle -> Varchar,
        otp -> Varchar,
        session_token -> Varchar,
        verification_count -> Int4,
        is_verified -> Bool,
    }
}
