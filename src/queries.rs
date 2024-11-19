use chrono::NaiveDateTime;
use diesel::r2d2::PoolError as R2D2Error;
use diesel::result::Error as DieselError;
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel::{OptionalExtension, RunQueryDsl};
use std::fmt;
use uuid::Uuid;

use crate::models::{NewUser, Session};
use crate::schema::sessions::{self};
use crate::{models::User, schema::users, DbPool};

/// Custom error type for database interactions.
///
/// This error type represents possible issues when interacting with the database, including:
/// - `ConnectionError`: Issues with the connection pool.
/// - `QueryError`: Errors returned by Diesel during query execution.
#[derive(Debug)]
pub enum DbError {
    ConnectionError(R2D2Error),
    QueryError(DieselError),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::ConnectionError(e) => write!(f, "Database connection error: {}", e),
            DbError::QueryError(e) => write!(f, "Query execution error: {}", e),
        }
    }
}

impl std::error::Error for DbError {}

impl From<R2D2Error> for DbError {
    fn from(err: R2D2Error) -> Self {
        DbError::ConnectionError(err)
    }
}

impl From<DieselError> for DbError {
    fn from(err: DieselError) -> Self {
        DbError::QueryError(err)
    }
}

/// Fetches a user by username.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `username`: The username to query.
///
/// # Returns
/// - `Ok(Some(User))`: If the user exists.
/// - `Ok(None)`: If no user matches the given username.
/// - `Err(DbError)`: If there is a database-related error.
pub fn get_user_by_username(pool: &DbPool, username: &str) -> Result<Option<User>, DbError> {
    let mut conn = pool.get()?;

    Ok(users::table
        .filter(users::username.eq(username))
        .first::<User>(&mut conn)
        .optional()?)
}

/// Creates a new session in the database.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `session_data`: The session details to insert.
///
/// # Returns
/// - `Ok(usize)`: The number of rows inserted (typically 1).
/// - `Err(DbError)`: If there is a database-related error.
pub fn create_new_session(pool: &DbPool, session_data: &Session) -> Result<usize, DbError> {
    let mut conn = pool.get()?;

    Ok(diesel::insert_into(sessions::table)
        .values(session_data)
        .execute(&mut conn)?)
}

/// Retrieves an active session by token and checks its expiry.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `token`: The session token to validate.
/// - `current_time`: The current time for expiry validation.
///
/// # Returns
/// - `Ok(Some(Session))`: If the session is valid and active.
/// - `Ok(None)`: If no session matches the given token or if it is expired.
/// - `Err(DbError)`: If there is a database-related error.
pub fn get_active_session_by_token(
    pool: &DbPool,
    token: &str,
    current_time: NaiveDateTime,
) -> Result<Option<Session>, DbError> {
    use crate::schema::sessions::dsl::*;

    let mut conn = pool.get()?;

    Ok(sessions
        .filter(session_token.eq(token))
        .filter(expires_at.gt(current_time))
        .first::<Session>(&mut conn)
        .optional()?)
}

/// Deletes a session by user ID and session token.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `user_id`: The user ID associated with the session.
/// - `token`: The session token to delete.
///
/// # Returns
/// - `Ok(usize)`: The number of rows deleted (typically 1).
/// - `Err(DbError)`: If there is a database-related error.
pub fn delete_session(pool: &DbPool, user_id: Uuid, token: &str) -> Result<usize, DbError> {
    let mut conn = pool.get()?;

    Ok(diesel::delete(
        sessions::table
            .filter(sessions::user_id.eq(user_id))
            .filter(sessions::session_token.eq(token)),
    )
    .execute(&mut conn)?)
}

/// Creates a new user in the database.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `user_data`: The user details to insert.
///
/// # Returns
/// - `Ok(User)`: The created user record.
/// - `Err(DbError)`: If there is a database-related error.
pub fn create_new_user(pool: &DbPool, user_data: &NewUser) -> Result<User, DbError> {
    let mut conn = pool.get()?;

    Ok(diesel::insert_into(users::table)
        .values(user_data)
        .get_result::<User>(&mut conn)?)
}
