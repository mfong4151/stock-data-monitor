// @generated automatically by Diesel CLI.

diesel::table! {
    stocks (id) {
        id -> Int4,
        ticker -> Varchar,
        open -> Float8,
        close -> Float8,
        high -> Float8,
        low -> Float8,
        ema_9 -> Float8,
        timestamp -> Int8,
        created_at -> Nullable<Timestamptz>,
    }
}
