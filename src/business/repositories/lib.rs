use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Error, Postgres, Transaction};
use std::time::Duration;
use tracing::{event, Level};

use environment::{owned_var, owned_var_or};
use tokio::{select, time};

/// # Loadable<T>
/// Represents the type of a T that can be loaded from the
/// database, where the query can either fail, resolve to None
/// or resolve to Some(T).
///
/// ## Variants
/// - Err(e)           query failed
/// - Ok(None)         T not found
/// - Ok(Some(T))      T found
pub type Loadable<T> = anyhow::Result<Option<T>>;

/// # Transaction
/// Represents a Postgres transaction
pub type AppTransaction = Transaction<'static, Postgres>;

lazy_static! {
    static ref DB_CONTEXT: AsyncOnce<Database> =
        AsyncOnce::new(Database::init());
}

pub struct Database {
    pool: PgPool,
}
impl Database {
    /// # Panics
    ///
    /// Panics when connection pool fails to initialize.
    async fn init() -> Self {
        let db_url: String = owned_var("DATABASE_URL");
        let conn_pool_max: u32 = owned_var_or("DB_CONN_POOL_MAX", 100);
        let pool = PgPoolOptions::new()
            .max_connections(conn_pool_max)
            .connect(&db_url)
            .await
            .unwrap_or_else(|e| {
                panic!("Failed to connect to Postgres DB! Error: {e}")
            });

        Self { pool }
    }

    pub async fn get_pool() -> &'static PgPool {
        &DB_CONTEXT.get().await.pool
    }

    /// # Errors
    ///
    /// Fails when a transaction cannot be started.
    pub async fn get_tx() -> Result<AppTransaction, Error> {
        DB_CONTEXT.get().await.pool.begin().await
    }

    #[allow(clippy::redundant_pub_crate)] // Select macro propagates this
    pub async fn disconnect() {
        let db_countdown = time::sleep(Duration::from_secs(15));
        let db_shutdown = async {
            event!(Level::INFO, "Closing database connections (max. 15s)...");
            DB_CONTEXT.get().await.pool.close().await;
            event!(Level::DEBUG, "All database connections closed!");
        };

        select! {
          () = db_countdown => {},
          () = db_shutdown => {},
        }
    }
}
