// Import PostgreSQL connection pool type from SQLx.
// PgPool manages a pool of database connections for efficient reuse.
use sqlx::PgPool;

/// Application state shared across handlers.
/// This struct is typically stored in Axum's `State<AppState>`
/// so every request handler can access the database pool.
#[derive(Clone)]
pub struct AppState {
    /// Shared PostgreSQL connection pool.
    /// Cloning AppState is cheap because PgPool internally uses Arc.
    pub pool: PgPool,
}

impl AppState {
    /// Initializes the application state.
    ///
    /// This function:
    /// 1. Reads database configuration from environment variables
    /// 2. Creates a connection pool
    /// 3. Runs database migrations
    /// 4. Returns a ready-to-use AppState
    pub async fn init() -> sqlx::Result<Self> {
        /// Read DATABASE_URL from environment variables.
        /// This must be set before starting the application.
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        /// Create a PostgreSQL connection pool.
        /// This pool will manage multiple connections automatically.
        let pool = PgPool::connect(&database_url).await?;

        // Run database migrations before the app starts.
        // This ensures the schema is up-to-date.
        sqlx::migrate("./migrations").run(&pool).await?;

        // Return initialized application state.
        Ok(Self { pool })
    }
}
