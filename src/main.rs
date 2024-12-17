/// Main module and entry point for the Actix Web server application.
///
/// This module sets up the application server, establishes a connection to the database,
/// and configures middleware for logging, CORS, and authentication. The server provides
/// both public and protected routes, and employs a structured `DbPool` for efficient
/// database access.
mod handlers;
mod middleware;
mod models;
mod queries;
mod renders;
mod requests;
mod schema;
mod services;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use middleware::AuthMiddleware;
use std::env;
use std::io::Write;
use tera::Tera;

/// Type alias for a Diesel connection pool for PostgreSQL, enabling shared access to database connections.
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Initializes the main application, setting up the Actix Web server with middleware, routing,
/// and database connection pooling. The server provides a set of routes with different access levels:
/// - Public: login route
/// - Protected: health check, logout, and user creation routes
///
/// # Returns
///
/// A `Result` indicating the success or failure of the application startup.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let tera = Tera::new("templates/**/*").unwrap();
    let base_url = if env::var("DEPLOY_PROD").unwrap_or_else(|_| "0".to_string()) == "1" {
        "/vimana2"
    } else {
        ""
    };

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(base_url.to_string()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            // .route("/", web::get().to(renders::render_login))
            // .route(
            //     "/home",
            //     web::get()
            //         .to(renders::render_home)
            //         .wrap(AuthMiddleware::new(pool.clone())),
            // )
            // .route(
            //     "/refuel",
            //     web::get()
            //         .to(renders::render_refuel)
            //         .wrap(AuthMiddleware::new(pool.clone())),
            // )
            // .route(
            //     "/vehicles/{vehicle_id}",
            //     web::get()
            //         .to(renders::render_vehicle)
            //         .wrap(AuthMiddleware::new(pool.clone())),
            // )
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/public")
                            .route("/login", web::post().to(handlers::login))
                            .route("/health", web::get().to(handlers::health_check)),
                    )
                    .service(
                        web::scope("/protected")
                            .wrap(AuthMiddleware::new(pool.clone()))
                            .route("/health", web::get().to(handlers::health_check))
                            .route("/logout", web::post().to(handlers::logout))
                            .route("/users", web::post().to(handlers::create_user))
                            .service(
                                web::scope("/vehicles")
                                    .route("/", web::post().to(handlers::create_vehicle))
                                    .route("/", web::get().to(handlers::get_vehicles_by_user))
                                    .route(
                                        "/{vehicle_id}",
                                        web::get().to(handlers::get_vehicle_by_id),
                                    )
                                    .route(
                                        "/{vehicle_id}",
                                        web::put().to(handlers::update_vehicle_by_id),
                                    )
                                    .route(
                                        "/{vehicle_id}",
                                        web::delete().to(handlers::delete_vehicle_by_id),
                                    ),
                            )
                            .service(
                                web::scope("/odometer")
                                    .route(
                                        "/{vehicle_id}",
                                        web::post().to(handlers::create_odometer),
                                    )
                                    .route(
                                        "/{vehicle_id}/latest",
                                        web::get().to(handlers::get_latest_odometer),
                                    )
                                    .route(
                                        "/{vehicle_id}/timeseries",
                                        web::get().to(handlers::get_odometer_timeseries),
                                    )
                                    .route(
                                        "/{vehicle_id}/traveled",
                                        web::get().to(handlers::get_traveled_distance),
                                    ),
                            )
                            .service(
                                web::scope("/refuel")
                                    .route("/{vehicle_id}", web::post().to(handlers::create_refuel))
                                    .route(
                                        "/{vehicle_id}/latest",
                                        web::get().to(handlers::get_latest_refuel),
                                    )
                                    .route(
                                        "/{vehicle_id}/timeseries",
                                        web::get().to(handlers::get_refuel_timeseries),
                                    ),
                            ),
                    ),
            )
            // .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(fs::Files::new("/", "./frontend/build/client").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
