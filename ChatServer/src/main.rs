mod db;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // get the environment variables from the .env file
    dotenvy::dotenv().ok();

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .init();

    // connect to the database
    let _db: db::Db = db::Db::new().await?;

    Ok(())
}
