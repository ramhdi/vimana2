use crate::middleware::AuthenticatedRequest;
use crate::models::NewVehicle;
use crate::requests::{
    DateIntervalRequest, LoginRequest, NewOdometerRequest, NewRefuelRequest, NewUserRequest,
    NewVehicleRequest, UpdateVehicleRequest,
};
use crate::{services, DbPool};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use uuid::Uuid;

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

    match services::login(&pool, &req).await {
        Ok(session_token) => Ok(HttpResponse::Ok()
            .cookie(
                actix_web::cookie::Cookie::build("session_token", session_token)
                    .path(base_url)
                    .http_only(true)
                    .max_age(actix_web::cookie::time::Duration::days(1))
                    .finish(),
            )
            .body("Logged in successfully")),
        Err(e) => Err(e.into()),
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

    match services::logout(&pool, user_id, &token).await {
        Ok(()) => Ok(HttpResponse::Ok()
            .cookie(
                actix_web::cookie::Cookie::build("session_token", "")
                    .path(base_url)
                    .http_only(true)
                    .max_age(actix_web::cookie::time::Duration::seconds(0))
                    .finish(),
            )
            .body("Logged out successfully")),
        Err(e) => Err(e.into()),
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

    match services::create_user(
        &pool,
        user_id,
        new_user.username.clone(),
        new_user.password.clone(),
        new_user.full_name.clone(),
    )
    .await
    {
        Ok(new_user) => Ok(HttpResponse::Created().body(new_user.id.to_string())),
        Err(e) => Err(e.into()),
    }
}

/// Handler to create a new vehicle.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `new_vehicle`: The new vehicle data.
/// - `req`: HTTP request object (for extracting user ID).
///
/// # Returns
/// - `201 Created` with the new vehicle data if successful.
/// - Appropriate HTTP error code if the operation fails.
pub async fn create_vehicle(
    pool: web::Data<DbPool>,
    new_vehicle: web::Json<NewVehicleRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let user_id = req
        .authenticated_user_id()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Internal server error"))?;

    let new_vehicle = new_vehicle.into_inner();
    let new_vehicle = NewVehicle {
        id: uuid::Uuid::new_v4(),
        brand: new_vehicle.brand,
        model: new_vehicle.model,
        registration: new_vehicle.registration,
        registration_expiry_date: new_vehicle.registration_expiry_date,
        user_id,
    };

    let vehicle_data = services::create_vehicle(&pool, &new_vehicle).await;

    match vehicle_data {
        Ok(vehicle) => Ok(HttpResponse::Created().json(vehicle)),
        Err(e) => Err(e.into()),
    }
}

/// Handler to get all vehicles owned by the authenticated user.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `req`: HTTP request object (for extracting user ID).
///
/// # Returns
/// - `200 OK` with a list of vehicles.
/// - Appropriate HTTP error code if the operation fails.
pub async fn get_vehicles_by_user(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let user_id = req
        .authenticated_user_id()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Internal server error"))?;

    match services::get_vehicles_by_user_id(&pool, user_id).await {
        Ok(vehicles) => Ok(HttpResponse::Ok().json(vehicles)),
        Err(e) => Err(e.into()),
    }
}

/// Handler to get a vehicle by its ID.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The vehicle ID.
///
/// # Returns
/// - `200 OK` with the vehicle data if found.
/// - Appropriate HTTP error code if the operation fails.
pub async fn get_vehicle_by_id(
    pool: web::Data<DbPool>,
    vehicle_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    match services::get_vehicle_by_id(&pool, vehicle_id.into_inner()).await {
        Ok(vehicle) => Ok(HttpResponse::Ok().json(vehicle)),
        Err(e) => Err(e.into()),
    }
}

/// Handler to update a vehicle by its ID.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The vehicle ID.
/// - `update_data`: The updated vehicle data.
///
/// # Returns
/// - `200 OK` with the updated vehicle data if successful.
/// - Appropriate HTTP error code if the operation fails.
pub async fn update_vehicle_by_id(
    pool: web::Data<DbPool>,
    vehicle_id: web::Path<Uuid>,
    update_data: web::Json<UpdateVehicleRequest>,
) -> Result<HttpResponse, Error> {
    let update_data = update_data.into_inner();
    match services::update_vehicle_by_id(
        &pool,
        vehicle_id.into_inner(),
        update_data.brand,
        update_data.model,
        update_data.registration,
        update_data.registration_expiry_date,
    )
    .await
    {
        Ok(vehicle) => Ok(HttpResponse::Ok().json(vehicle)),
        Err(e) => Err(e.into()),
    }
}

/// Handler to delete a vehicle by its ID.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The vehicle ID.
///
/// # Returns
/// - `204 No Content` if the vehicle was deleted successfully.
/// - Appropriate HTTP error code if the operation fails.
pub async fn delete_vehicle_by_id(
    pool: web::Data<DbPool>,
    vehicle_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    match services::delete_vehicle_by_id(&pool, vehicle_id.into_inner()).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Err(e.into()),
    }
}

/// Handler to create a new odometer entry.
///
/// This handler wraps the `create_new_odometer` service function.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
/// - `odometer_data`: The odometer data from the request.
///
/// # Returns
/// - `201 Created` with the created odometer data.
/// - Appropriate HTTP error code if the operation fails.
pub async fn create_odometer(
    pool: web::Data<DbPool>,
    vehicle_id: web::Path<Uuid>,
    odometer_data: web::Json<NewOdometerRequest>,
) -> Result<HttpResponse, Error> {
    let odometer_data = odometer_data.into_inner();

    match services::create_new_odometer(
        &pool,
        vehicle_id.into_inner(),
        odometer_data.odometer_value,
        odometer_data.timestamp,
    )
    .await
    {
        Ok(odometer) => Ok(HttpResponse::Created().json(odometer)),
        Err(e) => Err(e.into()),
    }
}

/// Handler to get the latest odometer entry for a vehicle.
///
/// This handler wraps the `get_latest_odometer` service function.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
///
/// # Returns
/// - `200 OK` with the latest odometer data.
/// - Appropriate HTTP error code if the operation fails.
pub async fn get_latest_odometer(
    pool: web::Data<DbPool>,
    vehicle_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    match services::get_latest_odometer(&pool, vehicle_id.into_inner()).await {
        Ok(odometer) => Ok(HttpResponse::Ok().json(odometer)),
        Err(e) => Err(e.into()),
    }
}

/// Handler to create a new refuel event.
///
/// This handler wraps the `create_new_refuel` service function.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
/// - `refuel_data`: The refuel data from the request.
///
/// # Returns
/// - `201 Created` with the created refuel data.
/// - Appropriate HTTP error code if the operation fails.
pub async fn create_refuel(
    pool: web::Data<DbPool>,
    vehicle_id: web::Path<Uuid>,
    refuel_data: web::Json<NewRefuelRequest>,
) -> Result<HttpResponse, Error> {
    let refuel_data = refuel_data.into_inner();

    match services::create_new_refuel(
        &pool,
        vehicle_id.into_inner(),
        refuel_data.refuel_quantity,
        refuel_data.odometer_value,
        refuel_data.timestamp,
    )
    .await
    {
        Ok(refuel) => Ok(HttpResponse::Created().json(refuel)),
        Err(e) => Err(e.into()),
    }
}

/// Handler to get the latest refuel event for a vehicle.
///
/// This handler wraps the `get_latest_refuel` service function.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
///
/// # Returns
/// - `200 OK` with the latest refuel data.
/// - Appropriate HTTP error code if the operation fails.
pub async fn get_latest_refuel(
    pool: web::Data<DbPool>,
    vehicle_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    match services::get_latest_refuel(&pool, vehicle_id.into_inner()).await {
        Ok(refuel) => Ok(HttpResponse::Ok().json(refuel)),
        Err(e) => Err(e.into()),
    }
}

/// Handler to get time-series odometer data for a vehicle.
///
/// This handler wraps the `get_odometer_timeseries` service function.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
/// - `start_date`: Start of the date range.
/// - `end_date`: End of the date range.
///
/// # Returns
/// - `200 OK` with the time-series odometer data.
/// - Appropriate HTTP error code if the operation fails.
pub async fn get_odometer_timeseries(
    pool: web::Data<DbPool>,
    vehicle_id: web::Path<Uuid>,
    query: web::Query<(NaiveDateTime, NaiveDateTime)>,
) -> Result<HttpResponse, Error> {
    let (start_date, end_date) = query.into_inner();

    match services::get_odometer_timeseries(&pool, vehicle_id.into_inner(), start_date, end_date)
        .await
    {
        Ok(odometer_data) => Ok(HttpResponse::Ok().json(odometer_data)),
        Err(e) => Err(e.into()),
    }
}

/// Handler to get time-series refuel data for a vehicle.
///
/// This handler wraps the `get_refuel_timeseries` service function.
///
/// # Arguments
/// - `pool`: Database connection pool.
/// - `vehicle_id`: The ID of the vehicle.
/// - `start_date`: Start of the date range.
/// - `end_date`: End of the date range.
///
/// # Returns
/// - `200 OK` with the time-series refuel data.
/// - Appropriate HTTP error code if the operation fails.
pub async fn get_refuel_timeseries(
    pool: web::Data<DbPool>,
    vehicle_id: web::Path<Uuid>,
    query: web::Query<(NaiveDateTime, NaiveDateTime)>,
) -> Result<HttpResponse, Error> {
    let (start_date, end_date) = query.into_inner();

    match services::get_refuel_timeseries(&pool, vehicle_id.into_inner(), start_date, end_date)
        .await
    {
        Ok(refuel_data) => Ok(HttpResponse::Ok().json(refuel_data)),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_traveled_distance(
    pool: web::Data<DbPool>,
    vehicle_id: web::Path<Uuid>,
    query: web::Query<DateIntervalRequest>,
) -> Result<HttpResponse, Error> {
    let request = query.into_inner();

    match services::get_traveled_distance(
        &pool,
        vehicle_id.into_inner(),
        request.start_date,
        request.end_date,
    )
    .await
    {
        Ok(refuel_data) => Ok(HttpResponse::Ok().json(refuel_data)),
        Err(e) => Err(e.into()),
    }
}
