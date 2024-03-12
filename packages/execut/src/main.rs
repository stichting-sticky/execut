use axum::{
    routing::{any, post},
    Router,
};
use execut::{handlers, Context, Keys};
use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("environment variable `DATABASE_URL` must be set");

    let pool = PgPool::connect_lazy(&database_url)
        .expect("unable to connect to postgres database, ensure it is running");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("unable to run migrations on postgres database");

    let secret =
        std::env::var("JWT_SECRET").expect("environment variable `JWT_SECRET` must be set");

    let keys = Keys::new(secret.as_bytes());

    let state = Context { pool, keys };

    let api = Router::new()
        .route("/health", any(handlers::health_check))
        .route("/auth", post(handlers::authorize))
        .route("/populate", post(handlers::populate))
        .with_state(state);

    let app = Router::new().nest("/v1", api);

    let addr = "127.0.0.1:3000";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
