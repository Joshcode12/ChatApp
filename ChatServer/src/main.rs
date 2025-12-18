mod api;
mod db;
mod error;
mod models;
mod routes;
mod middleware;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load the environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .init();

    // get the database url
    let database_url: String = std::env::var("DATABASE_URL").expect("Database url must be set");

    tracing::info!("Connecting to database...");
    let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    tracing::info!("Connected to database.");

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { db: pool };

    // Build router
    let app: axum::Router = axum::Router::new()
        .nest("/chat", routes::routes())
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state);

    let addr: std::net::SocketAddr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
