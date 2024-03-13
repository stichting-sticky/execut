use axum::{
    routing::{any, get, post},
    Router,
};
use execut::{handlers, Context, Keys};
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "execut=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let api = Router::new()
        .route("/health", any(handlers::health_check))
        .route("/auth", post(handlers::authorize))
        .route("/populate", post(handlers::populate))
        .route("/scans", get(handlers::get_scans))
        .route("/scans/:badge", post(handlers::scan_badge))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let app = Router::new().nest("/v1", api);

    let addr = "127.0.0.1:3000";

    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on {} 🚀", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
