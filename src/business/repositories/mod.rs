use futures::executor;
use lazy_static::lazy_static;
use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::configs::environment::{DATABASE_URL, MAX_CONN_POOL};

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

pub struct Context {
    pub pool: PgPool,
}

impl Context {
    /// # Panics
    ///
    /// Panics when connection pool fails to initialize.
    async fn init() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(*MAX_CONN_POOL)
            .connect(&DATABASE_URL)
            .await
            .unwrap_or_else(|e| {
                panic!("Failed to connect to Postgres DB! Error: {e}")
            });

        Self { pool }
    }
}

lazy_static! {
    pub static ref DB_CONTEXT: Context = executor::block_on(Context::init());
}
