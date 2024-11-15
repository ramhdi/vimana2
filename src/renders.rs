use actix_web::{web, HttpResponse, Responder};
use tera::{Context, Tera};

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

/// Renders the home (landing) page.
///
/// This handler uses the Tera templating engine to render the `home.html` template.
/// A dynamic base URL is passed to the template via the `base_url` context variable.
///
/// # Arguments
/// * `tera` - Shared instance of the Tera templating engine.
/// * `base_url` - The base URL for API requests or asset paths.
///
/// # Returns
/// An `HttpResponse` containing the rendered home page.
pub async fn render_home(tera: web::Data<Tera>, base_url: web::Data<String>) -> impl Responder {
    let mut context = Context::new();
    context.insert("base_url", &base_url.as_str());

    match tera.render("home.html", &context) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error rendering template: {e}"))
        }
    }
}
