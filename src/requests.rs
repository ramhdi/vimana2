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
