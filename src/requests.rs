use chrono::NaiveDate;
use serde::Deserialize;

/// Represents a login request with credentials provided by the user.
///
/// This struct is used to parse incoming login requests, containing:
/// - `username`: The unique username for the user attempting to log in.
/// - `password`: The plain-text password for authentication. This should be securely hashed
///   and compared against stored hashes in the authentication flow.
#[derive(Deserialize)]
pub struct LoginRequest {
    /// Username of the user attempting to log in.
    pub username: String,
    /// Plain-text password for authentication.
    pub password: String,
}

/// Represents a request to create a new user, containing the necessary details for registration.
///
/// This struct is used when a superuser creates a new user, and it contains:
/// - `username`: Desired unique username for the new user.
/// - `password`: Plain-text password, which will be hashed before storage.
/// - `full_name`: The full name of the user, for display and identification purposes.
#[derive(Deserialize)]
pub struct NewUserRequest {
    /// Desired unique username for the new user.
    pub username: String,
    /// Plain-text password for the new user, which will be hashed before storage.
    pub password: String,
    /// Full name of the user, used for display and identification purposes.
    pub full_name: String,
}

/// Represents a request to create a new vehicle.
///
/// This struct is used to parse incoming requests for creating vehicles, containing:
/// - `brand`: Brand name of the vehicle.
/// - `model`: Model name of the vehicle.
/// - `registration`: Registration number for the vehicle.
/// - `registration_expiry_date`: Expiration date of the vehicle's registration.
#[derive(Deserialize)]
pub struct NewVehicleRequest {
    /// Brand name of the vehicle.
    pub brand: String,
    /// Model name of the vehicle.
    pub model: String,
    /// Registration number for the vehicle.
    pub registration: String,
    /// Expiration date of the vehicle's registration.
    pub registration_expiry_date: NaiveDate,
}

/// Represents a request to update an existing vehicle.
///
/// This struct is used to parse incoming requests for updating vehicle details, containing:
/// - `brand`: Optional updated brand name.
/// - `model`: Optional updated model name.
/// - `registration`: Optional updated registration number.
/// - `registration_expiry_date`: Optional updated expiration date for the vehicle's registration.
#[derive(Deserialize)]
pub struct UpdateVehicleRequest {
    /// Updated brand name (optional).
    pub brand: Option<String>,
    /// Updated model name (optional).
    pub model: Option<String>,
    /// Updated registration number (optional).
    pub registration: Option<String>,
    /// Updated expiration date for the registration (optional).
    pub registration_expiry_date: Option<NaiveDate>,
}

/// Represents a request to create a new odometer entry.
///
/// This struct is used to parse incoming requests for creating odometer entries, containing:
/// - `odometer_value`: The odometer reading.
/// - `timestamp`: Optional timestamp for the entry.
#[derive(Deserialize)]
pub struct NewOdometerRequest {
    /// Odometer reading for the vehicle.
    pub odometer_value: f32,
    /// Optional timestamp for the odometer entry.
    pub timestamp: Option<chrono::NaiveDateTime>,
}

/// Represents a request to create a new refuel event.
///
/// This struct is used to parse incoming requests for creating refuel events, containing:
/// - `refuel_quantity`: The amount of fuel refueled.
/// - `odometer_value`: The odometer reading at the time of refueling.
/// - `timestamp`: Optional timestamp for the refuel event.
#[derive(Deserialize)]
pub struct NewRefuelRequest {
    /// The amount of fuel refueled.
    pub refuel_quantity: f32,
    /// The odometer reading at the time of refueling.
    pub odometer_value: f32,
    /// Optional timestamp for the refuel event.
    pub timestamp: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct DateIntervalRequest {
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}
