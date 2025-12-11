#[derive(Clone, Debug)]
pub struct Db {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

/// Database instance creation
impl Db {
    
    /// Connect to postgres database and run the migrations on it
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url: String =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

        tracing::info!("Connecting to database...");
        let pool: sqlx::pool::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await?;
        tracing::info!("Connected to database.");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;

        Ok(Self { pool })
    }
}
