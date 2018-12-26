// TODO: Silence this until diesel 1.4.
// See https://github.com/diesel-rs/diesel/issues/1785#issuecomment-422579609.
#![allow(proc_macro_derive_resolution_fallback)]

table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Varchar,
        password_hash -> Bytea,
        current_auth_token -> Nullable<Varchar>,
        last_action -> Nullable<Timestamp>,
    }
}
