use actix_web::{web, HttpRequest, HttpResponse, Responder};
use tera::{Context, Tera};

use crate::middleware::AuthenticatedRequest;
use crate::models::Vehicle;
use crate::services;

/// Renders the login page.
///
/// This handler uses the Tera templating engine to render the `login.html` template.
/// A dynamic base URL is passed to the template via the `base_url` context variable.
///
/// # Arguments
/// * `tera` - Shared instance of the Tera templating engine.
/// * `base_url` - The base URL for API requests or asset paths.
///
/// # Returns
/// An `HttpResponse` containing the rendered login page.
pub async fn render_login(tera: web::Data<Tera>, base_url: web::Data<String>) -> impl Responder {
    let mut context = Context::new();
    context.insert("base_url", &base_url.as_str());

    match tera.render("login.html", &context) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error rendering template: {e}"))
        }
    }
}

/// Renders the home (landing) page showing the user's vehicles in a grid layout.
///
/// This handler uses the Tera templating engine to render the `home.html` template.
/// Vehicle data is fetched from the database and passed to the template.
/// A dynamic base URL is passed to the template via the `base_url` context variable.
///
/// # Arguments
/// * `tera` - Shared instance of the Tera templating engine.
/// * `base_url` - The base URL for API requests or asset paths.
/// * `req` - The authenticated request to extract the user ID.
///
/// # Returns
/// An `HttpResponse` containing the rendered home page.
pub async fn render_home(
    tera: web::Data<Tera>,
    base_url: web::Data<String>,
    pool: web::Data<crate::DbPool>,
    req: HttpRequest,
) -> impl Responder {
    let mut context = Context::new();
    context.insert("base_url", &base_url.as_str());

    // Get authenticated user ID from request extensions
    let user_id = match req.authenticated_user_id() {
        Some(uid) => uid,
        None => {
            return HttpResponse::Unauthorized().body("Unauthorized access. Please log in again.");
        }
    };

    // Fetch vehicles for the user
    let vehicles: Vec<Vehicle> = match services::get_vehicles_by_user_id(&pool, user_id).await {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error fetching vehicles: {}", e));
        }
    };

    // Insert vehicles into the context for rendering
    context.insert("vehicles", &vehicles);

    // Render the home page with vehicle data
    match tera.render("home.html", &context) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error rendering template: {e}"))
        }
    }
}
