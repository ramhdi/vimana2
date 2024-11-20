use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{prelude::Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user in the system with essential identification and profile information.
///
/// This struct maps to the `users` table and contains fields for user-specific data, including:
/// - `id`: Unique identifier for the user.
/// - `username`: The user's unique username.
/// - `hashed_password`: The user's hashed password for authentication.
/// - `full_name`: The user's full name for display purposes.
/// - `created_at` and `updated_at`: Optional timestamps for record tracking.
#[derive(Selectable, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    /// Unique identifier for the user.
    pub id: Uuid,
    /// Username chosen by the user, must be unique across the system.
    pub username: String,
    /// Hashed password stored for authentication.
    #[serde(skip_serializing)]
    pub hashed_password: String,
    /// Full name of the user, used for display purposes.
    pub full_name: String,
    /// Timestamp for when the user record was created.
    pub created_at: Option<NaiveDateTime>,
    /// Timestamp for the last update made to the user record.
    pub updated_at: Option<NaiveDateTime>,
}

/// Represents a new user to be inserted into the database.
///
/// This struct is used during the creation of new users and does not require existing `created_at`
/// or `updated_at` fields, as these are managed automatically by the database.
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    /// Unique identifier for the new user.
    pub id: Uuid,
    /// Desired username for the new user.
    pub username: String,
    /// Hashed password for the new user.
    pub hashed_password: String,
    /// Full name of the new user.
    pub full_name: String,
    /// Creation timestamp for the new user record, optional as it's managed by the database.
    pub created_at: Option<NaiveDateTime>,
    /// Update timestamp for the new user record, optional as it's managed by the database.
    pub updated_at: Option<NaiveDateTime>,
}

/// Represents a session for user authentication and authorization.
///
/// This struct maps to the `sessions` table and contains fields related to a specific user session,
/// including:
/// - `id`: Unique identifier for the session.
/// - `user_id`: ID of the user associated with the session, nullable to allow anonymous sessions if necessary.
/// - `session_token`: Unique token identifying the session, stored securely.
/// - `expires_at`: Expiration timestamp for session validity.
/// - `created_at`: Timestamp of session creation, useful for tracking purposes.
#[derive(Selectable, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = sessions)]
pub struct Session {
    /// Unique identifier for the session.
    pub id: Uuid,
    /// Optional reference to the user ID associated with the session.
    pub user_id: Option<Uuid>,
    /// Secure token associated with the session for identification.
    pub session_token: String,
    /// Expiration timestamp indicating when the session becomes invalid.
    pub expires_at: NaiveDateTime,
    /// Timestamp for when the session record was created.
    pub created_at: Option<NaiveDateTime>,
}

/// Represents a new session to be inserted into the database.
///
/// This struct is used for creating new sessions and includes information required at the time
/// of session initialization.
#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    /// Unique identifier for the new session.
    pub id: Uuid,
    /// Optional reference to the user ID associated with the new session.
    pub user_id: Option<Uuid>,
    /// Unique session token generated at session initialization.
    pub session_token: String,
    /// Expiration timestamp for session validity.
    pub expires_at: NaiveDateTime,
    /// Timestamp for when the session is created, useful for tracking.
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Selectable, Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = vehicles)]
pub struct Vehicle {
    pub id: Uuid,
    pub brand: String,
    pub model: String,
    pub registration: String,
    pub registration_expiry_date: chrono::NaiveDate,
    pub user_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = vehicles)]
pub struct NewVehicle {
    pub id: Uuid,
    pub brand: String,
    pub model: String,
    pub registration: String,
    pub registration_expiry_date: chrono::NaiveDate,
    pub user_id: Uuid,
}

/// Represents an odometer reading associated with a vehicle.
///
/// This struct maps to the `odometer` table and stores information about the odometer value at a specific point in time.
/// - `id`: Unique identifier for the odometer record.
/// - `vehicle_id`: The vehicle associated with the odometer reading.
/// - `timestamp`: The time the odometer reading was recorded.
/// - `odometer_value`: The recorded odometer value in kilometers or miles.
/// - `created_at` and `updated_at`: Optional timestamps for record creation and updates.
#[derive(Selectable, Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Vehicle))]
#[diesel(table_name = odometer)]
pub struct Odometer {
    /// Unique identifier for the odometer record.
    pub id: Uuid,
    /// Vehicle ID associated with this odometer reading.
    pub vehicle_id: Uuid,
    /// Timestamp of the odometer reading.
    pub timestamp: Option<NaiveDateTime>,
    /// The odometer value recorded.
    pub odometer_value: f32,
    /// Timestamp for when the odometer record was created.
    pub created_at: Option<NaiveDateTime>,
    /// Timestamp for the last update made to the odometer record.
    pub updated_at: Option<NaiveDateTime>,
}

/// Represents a new odometer entry to be inserted into the database.
///
/// This struct is used during the creation of a new odometer record and does not require `created_at`
/// or `updated_at` fields, as these are managed automatically by the database.
#[derive(Insertable)]
#[diesel(table_name = odometer)]
pub struct NewOdometer {
    /// Vehicle ID associated with this new odometer reading.
    pub vehicle_id: Uuid,
    /// The odometer value recorded.
    pub odometer_value: f32,
}

/// Represents a refueling event associated with an odometer reading.
///
/// This struct maps to the `refuel` table and contains fields related to a specific refueling event:
/// - `id`: Unique identifier for the refueling event.
/// - `vehicle_id`: The vehicle associated with the refueling.
/// - `odometer_id`: The associated odometer reading for this refueling.
/// - `timestamp`: The time the refueling occurred.
/// - `refuel_quantity`: The amount of fuel added in liters or gallons.
/// - `created_at` and `updated_at`: Optional timestamps for record creation and updates.
#[derive(Selectable, Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Odometer))]
#[diesel(table_name = refuel)]
pub struct Refuel {
    /// Unique identifier for the refueling event.
    pub id: Uuid,
    /// Vehicle ID associated with this refueling.
    pub vehicle_id: Uuid,
    /// Odometer ID linked to this refueling.
    pub odometer_id: Uuid,
    /// Timestamp of the refueling event.
    pub timestamp: Option<NaiveDateTime>,
    /// Quantity of fuel added.
    pub refuel_quantity: f32,
    /// Timestamp for when the refueling record was created.
    pub created_at: Option<NaiveDateTime>,
    /// Timestamp for the last update made to the refueling record.
    pub updated_at: Option<NaiveDateTime>,
}

/// Represents a new refueling entry to be inserted into the database.
///
/// This struct is used during the creation of a new refueling record and does not require `created_at`
/// or `updated_at` fields, as these are managed automatically by the database.
#[derive(Insertable)]
#[diesel(table_name = refuel)]
pub struct NewRefuel {
    /// Vehicle ID associated with this new refueling.
    pub vehicle_id: Uuid,
    /// Odometer ID linked to this refueling.
    pub odometer_id: Uuid,
    /// Quantity of fuel added.
    pub refuel_quantity: f32,
}

/// Represents a detailed refueling event, including associated odometer information.
///
/// This struct is used for queries that require joined data from the `refuel`
/// and `odometer` tables.
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct RefuelWithOdometer {
    pub id: Uuid,
    pub vehicle_id: Uuid,
    pub odometer_id: Uuid,
    pub refuel_quantity: f32,
    pub odometer_value: f32,
    pub timestamp: Option<NaiveDateTime>,
}
