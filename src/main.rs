mod api;
mod config;
mod error;
mod models;
mod schema;
mod store;

use std::fs;

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{App, HttpServer, http, middleware::Logger, web};
use store::AppState;

async fn index(app_serve_path: web::Data<String>) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(format!(
        "{}/index.html",
        app_serve_path.as_ref()
    ))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cfg = config::Config::from_env()?;
    if cfg.jwt_secret == "debug-key" {
        log::warn!(
            "================================================================================"
        );
        log::warn!("JWT_SECRET is not configured! DO NOT use this in a deployment!");
        log::warn!(
            "================================================================================"
        );
    }
    if cfg.cors_allowed_origins == "*" {
        log::warn!(
            "================================================================================"
        );
        log::warn!("CORS_ALLOWED_ORIGINS is not configured! DO NOT use this in a deployment!");
        log::warn!(
            "================================================================================"
        );
    }
    let host = cfg.host.clone();
    let port = cfg.port;
    let app_serve_path_into = cfg.app_serve_path.clone();

    let state = web::Data::new(AppState::new(cfg)?);

    log::info!("Starting API on {host}:{port}");
    if let Some(app_serve_path) = state.config.app_serve_path.clone() {
        if fs::exists(&app_serve_path).unwrap_or(false) {
            log::info!("Hosting files stored at {}", app_serve_path);
        } else {
            log::info!(
                "Directory at {} does not exist. Files will not be hosted.",
                app_serve_path
            )
        }
    }

    let allowed_origins = state.config.parse_cors_allowed_origins();
    log::info!("Allowed origins: {:?}", allowed_origins);

    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                http::header::CONTENT_TYPE,
                http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(3600);
        for allowed_origin in &allowed_origins {
            if allowed_origin.contains("*") {
                cors = cors.send_wildcard();
            } else {
                cors = cors.allowed_origin(allowed_origin);
            }
        }

        let app_serve_path_into = app_serve_path_into.clone();
        let api = web::scope("/api/v1")
            .app_data(state.clone())
            .wrap(Logger::default())
            .route("/login", web::post().to(api::auth::login_post))
            .route("/login", web::get().to(api::auth::login_get))
            .route("/login", web::put().to(api::auth::login_put))
            .route("/refresh", web::post().to(api::auth::refresh_post))
            .route("/logout", web::post().to(api::auth::logout_post))
            .route("/user/{id}", web::get().to(api::user::user_get))
            .route("/user", web::post().to(api::user::user_post))
            .route("/user", web::delete().to(api::user::user_delete))
            .route("/prefs", web::get().to(api::prefs::prefs_all_get))
            .route("/prefs/scopes", web::get().to(api::prefs::scopes_get))
            .route("/prefs/{scope}", web::get().to(api::prefs::keys_get))
            .route("/prefs/{scope}/{key}", web::get().to(api::prefs::prefs_get))
            .route(
                "/prefs/{scope}/{key}",
                web::post().to(api::prefs::prefs_post),
            )
            .route("/prefs/{scope}/{key}", web::put().to(api::prefs::prefs_put))
            .route(
                "/prefs/{scope}/{key}",
                web::delete().to(api::prefs::prefs_delete),
            );

        if let Some(app_serve_path) = app_serve_path_into
            && fs::exists(&app_serve_path).unwrap_or(false)
        {
            App::new()
                .service(api)
                .app_data(app_serve_path.clone())
                .service(Files::new("/", app_serve_path).index_file("index.html"))
                .default_service(web::get().to(index))
                .wrap(cors)
        } else {
            App::new().service(api).wrap(cors)
        }
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
