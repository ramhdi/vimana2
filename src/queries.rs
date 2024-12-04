use chrono::{NaiveDate, NaiveDateTime};
use diesel::r2d2::PoolError as R2D2Error;
use diesel::result::Error as DieselError;
use diesel::sql_types::{Timestamptz, Uuid as DieselUuid};
use diesel::{ExpressionMethods, QueryDsl};
use diesel::{OptionalExtension, RunQueryDsl};
use std::fmt;
use uuid::Uuid;

use crate::{models, schema, DbPool};

/// Custom error type for database interactions.
///
/// This error type represents possible issues when interacting with the database, including:
/// - `ConnectionError`: Issues with the connection pool.
/// - `QueryError`: Errors returned by Diesel during query execution.
#[derive(Debug)]
pub enum DbError {
    ConnectionError(R2D2Error),
    QueryError(DieselError),
    StdError(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::ConnectionError(e) => write!(f, "Database connection error: {}", e),
            DbError::QueryError(e) => write!(f, "Query execution error: {}", e),
            DbError::StdError(e) => write!(f, "{}", e),
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
pub fn get_user_by_username(
    pool: &DbPool,
    user_name: &str,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::users::dsl::*;

    let mut conn = pool.get()?;

    Ok(schema::users::table
        .filter(username.eq(user_name))
        .first::<models::User>(&mut conn)
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
pub fn create_new_session(
    pool: &DbPool,
    session_data: &models::NewSession,
) -> Result<usize, DbError> {
    let mut conn = pool.get()?;

    Ok(diesel::insert_into(schema::sessions::table)
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
) -> Result<Option<models::Session>, DbError> {
    use crate::schema::sessions::dsl::*;

    let mut conn = pool.get()?;

    Ok(sessions
        .filter(session_token.eq(token))
        .filter(expires_at.gt(current_time))
        .first::<models::Session>(&mut conn)
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
        schema::sessions::table
            .filter(schema::sessions::user_id.eq(user_id))
            .filter(schema::sessions::session_token.eq(token)),
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
pub fn create_new_user(
    pool: &DbPool,
    user_data: &models::NewUser,
) -> Result<models::User, DbError> {
    let mut conn = pool.get()?;

    Ok(diesel::insert_into(schema::users::table)
        .values(user_data)
        .get_result::<models::User>(&mut conn)?)
}

/// Creates a new vehicle record in the database.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `new_vehicle`: The vehicle data to insert.
///
/// # Returns
/// - `Ok(Vehicle)`: The newly created vehicle.
/// - `Err(DbError)`: If the query fails.
pub fn create_vehicle(
    pool: &DbPool,
    new_vehicle: &models::NewVehicle,
) -> Result<models::Vehicle, DbError> {
    let mut conn = pool.get()?;
    Ok(diesel::insert_into(schema::vehicles::table)
        .values(new_vehicle)
        .get_result::<models::Vehicle>(&mut conn)?)
}

/// Retrieves all vehicles owned by a specific user.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `user_id`: The ID of the user.
///
/// # Returns
/// - `Ok(Vec<Vehicle>)`: A list of vehicles owned by the user.
/// - `Err(DbError)`: If the query fails.
pub fn get_vehicles_by_user_id(
    pool: &DbPool,
    user_id_value: Uuid,
) -> Result<Vec<models::Vehicle>, DbError> {
    use crate::schema::vehicles::dsl;

    let mut conn = pool.get()?;

    Ok(schema::vehicles::table
        .filter(dsl::user_id.eq(user_id_value))
        .order(dsl::created_at.asc())
        .load::<models::Vehicle>(&mut conn)?)
}

/// Retrieves a vehicle by its ID.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
///
/// # Returns
/// - `Ok(Some(Vehicle))`: The vehicle data, if found.
/// - `Ok(None)`: If no vehicle matches the given ID.
/// - `Err(DbError)`: If the query fails.
pub fn get_vehicle_by_id(
    pool: &DbPool,
    vehicle_id_value: Uuid,
) -> Result<Option<models::Vehicle>, DbError> {
    use crate::schema::vehicles::dsl::*;

    let mut conn = pool.get()?;

    Ok(schema::vehicles::table
        .filter(id.eq(vehicle_id_value))
        .first::<models::Vehicle>(&mut conn)
        .optional()?)
}

/// Updates a vehicle by its ID.
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
/// - `Err(DbError)`: If the query fails.
pub fn update_vehicle_by_id(
    pool: &DbPool,
    vehicle_id_value: Uuid,
    new_brand: Option<String>,
    new_model: Option<String>,
    new_registration: Option<String>,
    new_registration_expiry_date: Option<NaiveDate>,
) -> Result<models::Vehicle, DbError> {
    use crate::schema::vehicles::dsl::*;

    let mut conn = pool.get()?;

    diesel::update(vehicles.filter(id.eq(vehicle_id_value)))
        .set((
            new_brand.map(|val| brand.eq(val)),
            new_model.map(|val| model.eq(val)),
            new_registration.map(|val| registration.eq(val)),
            new_registration_expiry_date.map(|val| registration_expiry_date.eq(val)),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    get_vehicle_by_id(pool, vehicle_id_value)?
        .ok_or(DbError::QueryError(diesel::result::Error::NotFound))
}

/// Deletes a vehicle by its ID.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle to delete.
///
/// # Returns
/// - `Ok(usize)`: Number of rows deleted.
/// - `Err(DbError)`: If the query fails.
pub fn delete_vehicle_by_id(pool: &DbPool, vehicle_id_value: Uuid) -> Result<usize, DbError> {
    use crate::schema::vehicles::dsl::*;

    let mut conn = pool.get()?;

    Ok(diesel::delete(vehicles.filter(id.eq(vehicle_id_value))).execute(&mut conn)?)
}

/// Creates a new odometer entry for a specific vehicle.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
/// - `odometer_value`: The odometer reading.
/// - `timestamp`: Optional timestamp for the odometer entry.
///
/// # Returns
/// - `Ok(Odometer)`: The newly created odometer entry.
/// - `Err(DbError)`: If the query fails.
pub fn create_new_odometer(
    pool: &DbPool,
    vehicle_id: Uuid,
    odometer_value: f32,
    timestamp: Option<NaiveDateTime>,
) -> Result<models::Odometer, DbError> {
    use crate::schema::odometer;

    let mut conn = pool.get()?;
    let new_odometer = models::NewOdometer {
        vehicle_id,
        odometer_value,
    };

    diesel::insert_into(odometer::table)
        .values((
            &new_odometer,
            timestamp.map(|ts| odometer::timestamp.eq(ts)),
        ))
        .get_result::<models::Odometer>(&mut conn)
        .map_err(DbError::from)
}

/// Retrieves the latest odometer entry for a specific vehicle.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
///
/// # Returns
/// - `Ok(Some(Odometer))`: The latest odometer entry.
/// - `Ok(None)`: If no odometer entry exists.
/// - `Err(DbError)`: If the query fails.
pub fn get_latest_odometer(
    pool: &DbPool,
    vehicle_id: Uuid,
) -> Result<Option<models::Odometer>, DbError> {
    use crate::schema::odometer::dsl;

    let mut conn = pool.get()?;
    Ok(dsl::odometer
        .filter(dsl::vehicle_id.eq(vehicle_id))
        .order(dsl::timestamp.desc())
        .first::<models::Odometer>(&mut conn)
        .optional()?)
}

/// Retrieves time-series odometer data for a specific vehicle within a date range.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
/// - `start`: The start date/time of the range.
/// - `end`: The end date/time of the range.
///
/// # Returns
/// - `Ok(Vec<Odometer>)`: A list of odometer entries.
/// - `Err(DbError)`: If the query fails.
pub fn get_odometer_timeseries(
    pool: &DbPool,
    vehicle_id: Uuid,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<Vec<models::Odometer>, DbError> {
    use crate::schema::odometer::dsl;

    let mut conn = pool.get()?;
    Ok(dsl::odometer
        .filter(dsl::vehicle_id.eq(vehicle_id))
        .filter(dsl::timestamp.between(start, end))
        .order(dsl::timestamp.asc())
        .load::<models::Odometer>(&mut conn)?)
}

/// Creates a new refueling entry for a specific vehicle, along with an associated odometer entry.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
/// - `refuel_quantity`: The quantity of fuel.
/// - `odometer_value`: The odometer reading.
/// - `timestamp`: Optional timestamp for the refueling event.
///
/// # Returns
/// - `Ok(RefuelWithOdometer)`: The newly created refueling entry with odometer details.
/// - `Err(DbError)`: If the query fails.
pub fn create_new_refuel(
    pool: &DbPool,
    vehicle_id: Uuid,
    refuel_quantity: f32,
    odometer_value: f32,
    timestamp: Option<NaiveDateTime>,
) -> Result<models::RefuelWithOdometer, DbError> {
    use crate::schema::{odometer, refuel};
    use diesel::prelude::*;

    let mut conn = pool.get()?;

    conn.transaction(|conn| {
        let new_odometer = diesel::insert_into(odometer::table)
            .values((
                odometer::vehicle_id.eq(vehicle_id),
                odometer::odometer_value.eq(odometer_value),
                timestamp.map(|ts| odometer::timestamp.eq(ts)),
            ))
            .get_result::<models::Odometer>(conn)?;

        let new_refuel = diesel::insert_into(refuel::table)
            .values((
                refuel::vehicle_id.eq(vehicle_id),
                refuel::odometer_id.eq(new_odometer.id),
                refuel::refuel_quantity.eq(refuel_quantity),
                timestamp.map(|ts| refuel::timestamp.eq(ts)),
            ))
            .get_result::<models::Refuel>(conn)?;

        let result = models::RefuelWithOdometer {
            id: new_refuel.id,
            vehicle_id: new_refuel.vehicle_id,
            odometer_id: new_refuel.odometer_id,
            refuel_quantity: new_refuel.refuel_quantity,
            odometer_value: new_odometer.odometer_value,
            timestamp: new_refuel.timestamp,
        };

        Ok::<models::RefuelWithOdometer, DbError>(result)
    })
    .map_err(DbError::from)
}

/// Retrieves the latest refueling entry for a specific vehicle.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
///
/// # Returns
/// - `Ok(Some(RefuelWithOdometer))`: The latest refueling entry.
/// - `Ok(None)`: If no refueling entry exists.
/// - `Err(DbError)`: If the query fails.
pub fn get_latest_refuel(
    pool: &DbPool,
    vehicle_id: Uuid,
) -> Result<Option<models::RefuelWithOdometer>, DbError> {
    use crate::schema::{odometer, refuel};

    let mut conn = pool.get()?;
    Ok(refuel::table
        .inner_join(odometer::table)
        .filter(refuel::vehicle_id.eq(vehicle_id))
        .order(refuel::timestamp.desc())
        .select((
            refuel::id,
            refuel::vehicle_id,
            refuel::odometer_id,
            refuel::refuel_quantity,
            odometer::odometer_value,
            refuel::timestamp,
        ))
        .first::<models::RefuelWithOdometer>(&mut conn)
        .optional()?)
}

/// Retrieves time-series refueling data for a specific vehicle within a date range.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
/// - `start`: The start date/time of the range.
/// - `end`: The end date/time of the range.
///
/// # Returns
/// - `Ok(Vec<RefuelWithOdometer>)`: A list of refueling entries with odometer details.
/// - `Err(DbError)`: If the query fails.
pub fn get_refuel_timeseries(
    pool: &DbPool,
    vehicle_id: Uuid,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<Vec<models::RefuelWithOdometer>, DbError> {
    use crate::schema::{odometer, refuel};

    let mut conn = pool.get()?;
    Ok(refuel::table
        .inner_join(odometer::table)
        .filter(refuel::vehicle_id.eq(vehicle_id))
        .filter(refuel::timestamp.between(start, end))
        .order(refuel::timestamp.asc())
        .select((
            refuel::id,
            refuel::vehicle_id,
            refuel::odometer_id,
            refuel::refuel_quantity,
            odometer::odometer_value,
            refuel::timestamp,
        ))
        .load::<models::RefuelWithOdometer>(&mut conn)?)
}

pub fn get_traveled_distance(
    pool: &DbPool,
    vehicle_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Option<models::TraveledDistance>, DbError> {
    use diesel::sql_query;

    let mut conn = pool.get()?;

    let sql = r#"
        WITH first_reading AS (
            SELECT odometer_value
            FROM public.odometer
            WHERE vehicle_id = $1
            AND timestamp >= $2::timestamptz
            AND timestamp < ($3::timestamptz + interval '1 day')
            ORDER BY timestamp ASC
            LIMIT 1
        ),
        last_reading AS (
            SELECT odometer_value
            FROM public.odometer
            WHERE vehicle_id = $1
            AND timestamp >= $2::timestamptz
            AND timestamp < ($3::timestamptz + interval '1 day')
            ORDER BY timestamp DESC
            LIMIT 1
        )
        SELECT
            first_reading.odometer_value AS start_value,
            last_reading.odometer_value AS end_value,
            last_reading.odometer_value - first_reading.odometer_value AS traveled_distance
        FROM
            first_reading,
            last_reading
        ;
    "#;

    let start_date_bind = start_date.and_hms_opt(0, 0, 0).ok_or(DbError::StdError(
        "Error converting NaiveDate to NaiveDateTime".to_string(),
    ))?;
    let end_date_bind = end_date.and_hms_opt(0, 0, 0).ok_or(DbError::StdError(
        "Error converting NaiveDate to NaiveDateTime".to_string(),
    ))?;

    Ok(sql_query(sql)
        .bind::<DieselUuid, _>(vehicle_id)
        .bind::<Timestamptz, _>(start_date_bind)
        .bind::<Timestamptz, _>(end_date_bind)
        .get_result::<models::TraveledDistance>(&mut conn)
        .optional()?)
}
