use std::{env, sync::Arc, time::Duration};

use axum::{
    http::{header, Method},
    routing::get,
    Router,
};
use http::rest::auth;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer, MaxAge};

mod http;

struct AppStateInner {
    db: Pool<Postgres>,
}

type AppState = Arc<AppStateInner>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let postgres_user =
        env::var("POSTGRES_USER").expect("missing POSTGRES_USER environment variable");
    let postgres_password =
        env::var("POSTGRES_PASSWORD").expect("missing POSTGRES_PASSWORD environment variable");
    let postgres_database =
        env::var("POSTGRES_DB").expect("missing POSTGRES_DB environment variable");
    let connection_url = format!(
        "postgres://{}:{}@postgres/{}",
        postgres_user, postgres_password, postgres_database
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_url)
        .await
        .expect("failed to connect to postgres");

    let state = Arc::new(AppStateInner { db: pool });

    let app = Router::new()
        .route("/", get(root))
        .nest("/auth", auth::routes(state.clone()))
        .layer(cors_layer())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ]))
        .allow_headers(AllowHeaders::list([
            header::ACCEPT_ENCODING,
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::CONTENT_LENGTH,
        ]))
        .max_age(MaxAge::exact(Duration::from_secs(60) * 60))
}
