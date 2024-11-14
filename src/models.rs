use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
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
#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize)]
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

/// Represents a vehicle associated with a user.
///
/// This struct maps to the `vehicles` table, storing details about each registered vehicle.
/// - `id`: Unique identifier for the vehicle.
/// - `vehicle_name`: Name or model of the vehicle.
/// - `vehicle_registration`: Registration identifier for the vehicle.
/// - `user_id`: ID of the user who owns the vehicle, allowing nullable entries for vehicles without owners.
/// - `created_at` and `updated_at`: Optional timestamps for record tracking.
#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = vehicles)]
pub struct Vehicle {
    /// Unique identifier for the vehicle.
    pub id: Uuid,
    /// The name or model of the vehicle.
    pub vehicle_name: String,
    /// Registration number or identifier for the vehicle.
    pub vehicle_registration: String,
    /// Optional reference to the user ID who owns the vehicle.
    pub user_id: Option<Uuid>,
    /// Timestamp for when the vehicle record was created.
    pub created_at: Option<NaiveDateTime>,
    /// Timestamp for the last update made to the vehicle record.
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
#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize)]
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
