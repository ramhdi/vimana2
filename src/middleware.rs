use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header,
    Error, HttpMessage, HttpRequest,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref JWT_SECRET: Vec<u8> = {
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set")
            .into_bytes()
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // subject (user id)
    pub exp: usize, // expiration time (as UTC timestamp)
    pub iat: usize, // issued at (as UTC timestamp)
}

pub struct AuthMiddleware;

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

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
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

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // Extract bearer token from Authorization header
            let auth_header = req
                .headers()
                .get(header::AUTHORIZATION)
                .ok_or_else(|| ErrorUnauthorized("Missing authorization header"))?;

            let auth_str = auth_header
                .to_str()
                .map_err(|_| ErrorUnauthorized("Invalid authorization header"))?;

            if !auth_str.starts_with("Bearer ") {
                return Err(ErrorUnauthorized("Invalid authorization scheme"));
            }

            let token = &auth_str[7..]; // Skip "Bearer " prefix

            // Validate JWT token
            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(&JWT_SECRET),
                &Validation::default(),
            )
            .map_err(|_| ErrorUnauthorized("Invalid token"))?;

            // Insert user_id into request extensions
            req.extensions_mut().insert(token_data.claims.sub);

            service.call(req).await
        })
    }
}

/// Trait providing an extension for ServiceRequest and HttpRequest to access the authenticated user ID.
pub trait AuthenticatedRequest {
    /// Returns the authenticated user_id from the JWT claims.
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
