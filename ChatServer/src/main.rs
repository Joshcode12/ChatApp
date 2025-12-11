mod db;

#[tokio::main]
async fn main() -> () {
    // get the environment variables from the .env file
    dotenvy::dotenv().ok();

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .init();

    // connect to the database
    let _db: db::Db = db::Db::new().await.expect("Error with database.");

    // make the routes
    let app: axum::Router =
        axum::Router::new().route("/", axum::routing::get(|| async { "Hello World!" }));

    // make the socket
    let addr: std::net::SocketAddr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    // run the app
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c= async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate: std::future::Pending<()> = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
