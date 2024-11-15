use actix_web::{web, Error, HttpRequest, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};
use uuid::Uuid;

use crate::middleware::AuthenticatedRequest;
use crate::models::{NewUser, Session};
use crate::requests::{LoginRequest, NewUserRequest};
use crate::{queries, DbPool};

const SUPERUSER_ID_STR: &str = "7763abad-f33d-4308-b89d-8897e9037d16";
lazy_static::lazy_static! {
    /// Static reference to the superuser UUID, used for authorization checks.
    static ref SUPERUSER_ID: Uuid = Uuid::parse_str(SUPERUSER_ID_STR).expect("Invalid superuser ID format");
}

/// Health check handler that verifies server and database connectivity.
///
/// Returns:
/// - `200 OK` if the server and database are reachable.
/// - `503 ServiceUnavailable` if there is a database connection issue.
pub async fn health_check(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get();
    match conn {
        Ok(_) => HttpResponse::Ok().body("Server and database connection healthy"),
        Err(_) => HttpResponse::ServiceUnavailable().body("Database connection issue"),
    }
}

/// Handles user login by validating credentials and creating a session token.
///
/// This handler performs the following steps:
/// 1. Verifies the username and password.
/// 2. Creates a new session with an expiration date if credentials are valid.
/// 3. Responds with a `session_token` cookie and `200 OK` upon successful login.
///
/// Returns:
/// - `200 OK` with a session token if credentials are correct.
/// - `401 Unauthorized` if credentials are invalid.
pub async fn login(
    pool: web::Data<DbPool>,
    base_url: web::Data<String>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    let base_url = if base_url.is_empty() {
        "/"
    } else {
        base_url.as_str()
    };

    let user = queries::get_user_by_username(&pool, &req.username)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid username or password"))?;

    let is_valid = verify(&req.password, &user.hashed_password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password verification error"))?;

    match is_valid {
        true => {
            let session_token: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();

            let expires_at = Utc::now() + Duration::days(1);

            queries::create_new_session(
                &pool,
                &Session {
                    id: Uuid::new_v4(),
                    user_id: Some(user.id),
                    session_token: session_token.clone(),
                    expires_at: expires_at.naive_utc(),
                    created_at: Some(Utc::now().naive_utc()),
                },
            )
            .map_err(|_| {
                actix_web::error::ErrorInternalServerError(
                    "Failed to execute query to the database",
                )
            })?;

            Ok(HttpResponse::Ok()
                .cookie(
                    actix_web::cookie::Cookie::build("session_token", session_token)
                        .path(base_url)
                        .http_only(true)
                        .max_age(actix_web::cookie::time::Duration::days(1))
                        .finish(),
                )
                .body("Logged in successfully"))
        }

        false => Err(actix_web::error::ErrorUnauthorized(
            "Invalid username or password",
        )),
    }
}

/// Handles user logout by deleting the session token from the database and clearing the session cookie.
///
/// This handler performs the following steps:
/// 1. Validates that the `session_token` exists in the request cookies and matches an active session.
/// 2. Deletes the specific session from the database.
/// 3. Clears the `session_token` cookie and responds with `200 OK`.
///
/// Returns:
/// - `200 OK` on successful logout.
/// - `404 NotFound` if the session is not found.
/// - `401 Unauthorized` if the user is not authenticated.
pub async fn logout(
    pool: web::Data<DbPool>,
    base_url: web::Data<String>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let base_url = if base_url.is_empty() {
        "/"
    } else {
        base_url.as_str()
    };

    let user_id = req
        .authenticated_user_id()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Internal server error"))?;

    let token = req
        .cookie("session_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;

    let num_deleted = queries::delete_session(&pool, user_id, &token)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Error logging out"))?;

    match num_deleted {
        0 => Err(actix_web::error::ErrorNotFound("Session not found")),
        _ => Ok(HttpResponse::Ok()
            .cookie(
                actix_web::cookie::Cookie::build("session_token", "")
                    .path(base_url)
                    .http_only(true)
                    .max_age(actix_web::cookie::time::Duration::seconds(0))
                    .finish(),
            )
            .body("Logged out successfully")),
    }
}

/// Handles new user creation, restricted to the superuser.
///
/// This handler performs the following steps:
/// 1. Checks if the requester is the superuser.
/// 2. Hashes the new user's password and creates a new user record in the database.
/// 3. Responds with the created user details or appropriate errors.
///
/// Returns:
/// - `200 OK` with the new user details if creation is successful.
/// - `403 Forbidden` if the requester is not the superuser.
/// - `500 InternalServerError` if there is an error creating the user.
pub async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUserRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let user_id = req
        .authenticated_user_id()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Internal server error"))?;

    if user_id == *SUPERUSER_ID {
        let hashed_password = hash(&new_user.password, DEFAULT_COST)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to hash password"))?;

        let new_user = NewUser {
            id: Uuid::new_v4(),
            username: new_user.username.clone(),
            hashed_password,
            full_name: new_user.full_name.clone(),
            created_at: None,
            updated_at: None,
        };

        let new_user = queries::create_new_user(&pool, &new_user).map_err(|_| {
            actix_web::error::ErrorInternalServerError("Failed to execute query to the database")
        })?;

        Ok(HttpResponse::Created().body(new_user.id.to_string()))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    }
}
