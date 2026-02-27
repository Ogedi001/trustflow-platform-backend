//! Transaction wrapper for database interactions
//!
//! Provides a thin wrapper over `sqlx::Transaction` with error conversions.

#[cfg(feature = "database")]
use super::pool::DbPoolError;

#[cfg(feature = "database")]
use sqlx::Postgres;

/// Transaction wrapper
#[cfg(feature = "database")]
pub struct Transaction<'a> {
    tx: sqlx::Transaction<'a, Postgres>,
}

#[cfg(feature = "database")]
impl<'a> Transaction<'a> {
    /// Create a new transaction
    pub fn new(tx: sqlx::Transaction<'a, Postgres>) -> Self {
        Self { tx }
    }

    /// Commit the transaction
    pub async fn commit(self) -> Result<(), DbPoolError> {
        self.tx.commit().await.map_err(|e| e.into())
    }

    /// Rollback the transaction
    pub async fn rollback(self) -> Result<(), DbPoolError> {
        self.tx.rollback().await.map_err(|e| e.into())
    }

    /// Execute a query within the transaction
    pub async fn execute(
        &mut self,
        query: &str,
    ) -> Result<sqlx::postgres::PgQueryResult, DbPoolError> {
        sqlx::query(query)
            .execute(&mut *self.tx)
            .await
            .map_err(|e| e.into())
    }

    /// Fetch one row within the transaction
    pub async fn fetch_one<T>(&mut self, query: &str) -> Result<T, DbPoolError>
    where
        T: sqlx::FromRow<sqlx::postgres::PgRow>,
    {
        sqlx::query_as::<_, T>(query)
            .fetch_one(&mut *self.tx)
            .await
            .map_err(|e| e.into())
    }

    /// Fetch all rows within the transaction
    pub async fn fetch_all<T>(&mut self, query: &str) -> Result<Vec<T>, DbPoolError>
    where
        T: sqlx::FromRow<sqlx::postgres::PgRow>,
    {
        sqlx::query_as::<_, T>(query)
            .fetch_all(&mut *self.tx)
            .await
            .map_err(|e| e.into())
    }
}
