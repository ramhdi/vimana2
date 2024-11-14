use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, HttpMessage, HttpRequest,
};
use chrono::Utc;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use futures::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use uuid::Uuid;

use crate::models::Session;

/// Custom error type representing possible authentication errors in the middleware.
///
/// `AuthError` provides specific error types for various scenarios:
/// - `InvalidToken`: Session token is missing or invalid.
/// - `ExpiredSession`: Session has expired.
/// - `SessionNotFound`: Session does not exist in the database.
/// - `DatabaseError`: A general database error occurred.
#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    ExpiredSession,
    SessionNotFound,
    DatabaseError(diesel::result::Error),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidToken => write!(f, "Invalid session token"),
            AuthError::ExpiredSession => write!(f, "Session has expired"),
            AuthError::SessionNotFound => write!(f, "Session not found"),
            AuthError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl std::error::Error for AuthError {}

impl From<AuthError> for Error {
    fn from(err: AuthError) -> Error {
        ErrorUnauthorized(err.to_string())
    }
}

/// Middleware struct responsible for authenticating requests based on session tokens.
///
/// This middleware fetches the session token from cookies and verifies it against the database.
/// If valid, it inserts the `user_id` into the request's extensions for further use by handlers.
pub struct AuthMiddleware {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl AuthMiddleware {
    /// Creates a new instance of `AuthMiddleware` with a given database connection pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - Database connection pool to fetch sessions for authentication.
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    /// Creates the transformed `AuthMiddlewareService` to handle requests.
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            pool: self.pool.clone(),
        }))
    }
}

/// Service struct for processing each request, performing session validation and user ID injection.
///
/// This struct manages database access and error handling for each request as it flows
/// through the middleware chain.
pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    /// Processes the incoming request, extracting and validating the session token.
    ///
    /// Retrieves the `session_token` from the request's cookies and queries the database to verify
    /// the session. If valid and active, the associated `user_id` is inserted into the request's
    /// extensions for use in downstream handlers.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let pool = self.pool.clone();

        Box::pin(async move {
            let token = req
                .cookie("session_token")
                .ok_or_else(|| AuthError::InvalidToken)?
                .value()
                .to_string();

            // Establish a connection from the pool
            let mut conn = pool.get().map_err(|e| {
                AuthError::DatabaseError(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::Unknown,
                    Box::new(e.to_string()),
                ))
            })?;

            // Verify session in the database
            use crate::schema::sessions::dsl::*;
            let session = sessions
                .filter(session_token.eq(token))
                .filter(expires_at.gt(Utc::now().naive_utc()))
                .first::<Session>(&mut conn)
                .optional()
                .map_err(AuthError::DatabaseError)?;

            match session {
                Some(session) => {
                    if let Some(uid) = session.user_id {
                        req.extensions_mut().insert(uid);
                    } else {
                        return Err(ErrorUnauthorized("User ID missing in session"));
                    }
                    service.call(req).await
                }
                None => Err(AuthError::SessionNotFound.into()),
            }
        })
    }
}

/// Trait providing an extension for `ServiceRequest` and `HttpRequest` to access the authenticated user ID.
///
/// This trait allows for easy retrieval of the authenticated `user_id` (if present) from request
/// extensions, making it accessible across handlers and middleware.
pub trait AuthenticatedRequest {
    /// Returns the authenticated `user_id`, if available.
    fn authenticated_user_id(&self) -> Option<Uuid>;
}

impl AuthenticatedRequest for ServiceRequest {
    fn authenticated_user_id(&self) -> Option<Uuid> {
        self.extensions().get::<Uuid>().copied()
    }
}

impl AuthenticatedRequest for HttpRequest {
    fn authenticated_user_id(&self) -> Option<Uuid> {
        self.extensions().get::<Uuid>().copied()
    }
}
