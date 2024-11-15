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

pub fn get_user_by_username(pool: &DbPool, username: &str) -> Result<Option<User>, DbError> {
    let mut conn = pool.get()?;

    Ok(users::table
        .filter(users::username.eq(username))
        .first::<User>(&mut conn)
        .optional()?)
}

pub fn create_new_session(pool: &DbPool, session_data: &Session) -> Result<usize, DbError> {
    let mut conn = pool.get()?;

    Ok(diesel::insert_into(sessions::table)
        .values(session_data)
        .execute(&mut conn)?)
}

pub fn delete_session(pool: &DbPool, user_id: Uuid, token: &str) -> Result<usize, DbError> {
    let mut conn = pool.get()?;

    Ok(diesel::delete(
        sessions::table
            .filter(sessions::user_id.eq(user_id))
            .filter(sessions::session_token.eq(token)),
    )
    .execute(&mut conn)?)
}

pub fn create_new_user(pool: &DbPool, user_data: &NewUser) -> Result<User, DbError> {
    let mut conn = pool.get()?;

    Ok(diesel::insert_into(users::table)
        .values(user_data)
        .get_result::<User>(&mut conn)?)
}
