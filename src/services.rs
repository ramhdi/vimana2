use crate::{
    models::{NewUser, NewVehicle, Session, User, Vehicle},
    queries::{self, DbError},
    requests::LoginRequest,
    DbPool,
};
use actix_web::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, NaiveDate, Utc};
use rand::{distributions::Alphanumeric, Rng};
use std::fmt;
use uuid::Uuid;

/// Represents service-layer errors, including DB and application-specific issues.
///
/// This error type wraps database errors (`DbError`) and includes:
/// - `ValidationError`: For invalid user input (422).
/// - `Unauthorized`: For authentication failures (401).
/// - `Forbidden`: For access control violations (403).
/// - `NotFound`: For missing resources (404).
/// - `Other`: For unexpected or internal server errors (500).
#[derive(Debug)]
pub enum ServiceError {
    DbError(DbError),
    ValidationError(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Other(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::DbError(e) => write!(f, "Database error: {}", e),
            ServiceError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ServiceError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ServiceError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ServiceError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ServiceError::Other(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl std::error::Error for ServiceError {}

impl From<DbError> for ServiceError {
    fn from(err: DbError) -> Self {
        ServiceError::DbError(err)
    }
}

/// Converts `ServiceError` to Actix Web's `ResponseError` for HTTP responses.
///
/// This implementation maps service errors to appropriate HTTP status codes
/// and ensures consistent error messages.
impl actix_web::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY, // 422
            ServiceError::Unauthorized(_) => StatusCode::UNAUTHORIZED,            // 401
            ServiceError::Forbidden(_) => StatusCode::FORBIDDEN,                  // 403
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,                   // 404
            ServiceError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,          // 500
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

const SUPERUSER_ID_STR: &str = "7763abad-f33d-4308-b89d-8897e9037d16";

lazy_static::lazy_static! {
    /// Static reference to the superuser UUID, used for authorization checks.
    static ref SUPERUSER_ID: Uuid = Uuid::parse_str(SUPERUSER_ID_STR).expect("Invalid superuser ID format");
}

/// Handles user login by validating credentials and creating a session.
///
/// # Arguments
/// - `pool`: The database connection pool.
/// - `req`: The login request containing username and password.
///
/// # Returns
/// - `Ok(String)`: The session token upon successful login.
/// - `Err(ServiceError)`: If login fails due to invalid credentials or other issues.
pub async fn login(pool: &DbPool, req: &LoginRequest) -> Result<String, ServiceError> {
    let user = queries::get_user_by_username(pool, &req.username)?
        .ok_or_else(|| ServiceError::Unauthorized("Invalid username or password".to_string()))?;

    let is_valid = verify(&req.password, &user.hashed_password)
        .map_err(|_| ServiceError::Other("Password verification error".to_string()))?;

    if is_valid {
        let session_token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let expires_at = Utc::now() + Duration::days(1);

        queries::create_new_session(
            pool,
            &Session {
                id: Uuid::new_v4(),
                user_id: Some(user.id),
                session_token: session_token.clone(),
                expires_at: expires_at.naive_utc(),
                created_at: Some(Utc::now().naive_utc()),
            },
        )?;

        Ok(session_token)
    } else {
        Err(ServiceError::Unauthorized(
            "Invalid username or password".to_string(),
        ))
    }
}

/// Handles user logout by deleting a specific session.
///
/// # Arguments
/// - `pool`: The database connection pool.
/// - `user_id`: The ID of the user logging out.
/// - `token`: The session token to delete.
///
/// # Returns
/// - `Ok(())`: If the session was successfully deleted.
/// - `Err(ServiceError)`: If the session is not found or another error occurs.
pub async fn logout(pool: &DbPool, user_id: Uuid, token: &str) -> Result<(), ServiceError> {
    match queries::delete_session(pool, user_id, token)? {
        0 => Err(ServiceError::NotFound("Session not found".to_string())),
        _ => Ok(()),
    }
}

/// Handles the creation of a new user, restricted to superusers.
///
/// # Arguments
/// - `pool`: The database connection pool.
/// - `user_id`: The ID of the authenticated user (must be the superuser).
/// - `new_username`: The username for the new user.
/// - `new_password`: The password for the new user.
/// - `new_full_name`: The full name of the new user.
///
/// # Returns
/// - `Ok(User)`: The newly created user record.
/// - `Err(ServiceError)`: If the authenticated user is not a superuser or other issues occur.
pub async fn create_user(
    pool: &DbPool,
    user_id: Uuid,
    new_username: String,
    new_password: String,
    new_full_name: String,
) -> Result<User, ServiceError> {
    if user_id == *SUPERUSER_ID {
        let new_hashed_password = hash(&new_password, DEFAULT_COST)
            .map_err(|_| ServiceError::Other("Failed to hash password".to_string()))?;

        let new_user = NewUser {
            id: Uuid::new_v4(),
            username: new_username,
            hashed_password: new_hashed_password,
            full_name: new_full_name,
            created_at: None,
            updated_at: None,
        };

        let new_user = queries::create_new_user(pool, &new_user)?;

        Ok(new_user)
    } else {
        Err(ServiceError::Forbidden("Forbidden".to_string()))
    }
}

/// Service to create a new vehicle.
///
/// Validates the input and calls the `create_vehicle` query.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `new_vehicle`: The vehicle data to insert.
///
/// # Returns
/// - `Ok(Vehicle)`: The newly created vehicle.
/// - `Err(ServiceError)`: If the operation fails.
pub async fn create_vehicle(
    pool: &DbPool,
    new_vehicle: &NewVehicle,
) -> Result<Vehicle, ServiceError> {
    queries::create_vehicle(pool, new_vehicle).map_err(ServiceError::DbError)
}

/// Service to get all vehicles owned by a specific user.
///
/// Calls the `get_vehicles_by_user_id` query.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `user_id`: The ID of the user.
///
/// # Returns
/// - `Ok(Vec<Vehicle>)`: A list of vehicles owned by the user.
/// - `Err(ServiceError)`: If the operation fails.
pub async fn get_vehicles_by_user_id(
    pool: &DbPool,
    user_id: Uuid,
) -> Result<Vec<Vehicle>, ServiceError> {
    queries::get_vehicles_by_user_id(pool, user_id).map_err(ServiceError::DbError)
}

/// Service to get a vehicle by its ID.
///
/// Calls the `get_vehicle_by_id` query.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
///
/// # Returns
/// - `Ok(Vehicle)`: The vehicle data, if found.
/// - `Err(ServiceError::NotFound)`: If the vehicle does not exist.
/// - `Err(ServiceError)`: If the operation fails.
pub async fn get_vehicle_by_id(pool: &DbPool, vehicle_id: Uuid) -> Result<Vehicle, ServiceError> {
    queries::get_vehicle_by_id(pool, vehicle_id)?
        .ok_or_else(|| ServiceError::NotFound("Vehicle not found".to_string()))
}

/// Service to update a vehicle by its ID.
///
/// Calls the `update_vehicle_by_id` query and validates the input fields.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle to update.
/// - `new_brand`: Updated brand (optional).
/// - `new_model`: Updated model (optional).
/// - `new_registration`: Updated registration (optional).
/// - `new_registration_expiry_date`: Updated registration expiry date (optional).
///
/// # Returns
/// - `Ok(Vehicle)`: The updated vehicle data.
/// - `Err(ServiceError::NotFound)`: If the vehicle does not exist.
/// - `Err(ServiceError)`: If the operation fails.
pub async fn update_vehicle_by_id(
    pool: &DbPool,
    vehicle_id: Uuid,
    new_brand: Option<String>,
    new_model: Option<String>,
    new_registration: Option<String>,
    new_registration_expiry_date: Option<NaiveDate>,
) -> Result<Vehicle, ServiceError> {
    queries::update_vehicle_by_id(
        pool,
        vehicle_id,
        new_brand,
        new_model,
        new_registration,
        new_registration_expiry_date,
    )
    .map_err(ServiceError::DbError)
}

/// Service to delete a vehicle by its ID.
///
/// Calls the `delete_vehicle_by_id` query.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle to delete.
///
/// # Returns
/// - `Ok(())`: If the vehicle was deleted.
/// - `Err(ServiceError::NotFound)`: If the vehicle does not exist.
/// - `Err(ServiceError)`: If the operation fails.
pub async fn delete_vehicle_by_id(pool: &DbPool, vehicle_id: Uuid) -> Result<(), ServiceError> {
    match queries::delete_vehicle_by_id(pool, vehicle_id)? {
        0 => Err(ServiceError::NotFound("Vehicle not found".to_string())),
        _ => Ok(()),
    }
}
