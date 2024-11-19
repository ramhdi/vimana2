// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        session_token -> Text,
        expires_at -> Timestamptz,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        hashed_password -> Text,
        full_name -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    vehicles (id) {
        id -> Uuid,
        brand -> Text,
        model -> Text,
        registration -> Text,
        registration_expiry_date -> Date,
        user_id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(vehicles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
    vehicles,
);
