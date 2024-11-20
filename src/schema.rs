// @generated automatically by Diesel CLI.

diesel::table! {
    odometer (id) {
        id -> Uuid,
        vehicle_id -> Uuid,
        timestamp -> Nullable<Timestamptz>,
        odometer_value -> Float4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    refuel (id) {
        id -> Uuid,
        vehicle_id -> Uuid,
        odometer_id -> Uuid,
        timestamp -> Nullable<Timestamptz>,
        refuel_quantity -> Float4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

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

diesel::joinable!(odometer -> vehicles (vehicle_id));
diesel::joinable!(refuel -> odometer (odometer_id));
diesel::joinable!(refuel -> vehicles (vehicle_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(vehicles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    odometer,
    refuel,
    sessions,
    users,
    vehicles,
);
