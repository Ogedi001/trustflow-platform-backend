//! Database infrastructure for Identity Service
//!
//! SQLx database models, migrations, and connection management.

use sqlx::migrate::Migrator;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::path::Path;
use thiserror::Error;
use tracing::info;

/// Database errors
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Connection error: {0}")]
    Connection(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

/// Database connection pool
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn new(connection_string: &str, max_connections: u32) -> Result<Self, DbError> {
        info!("Connecting to database...");

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .min_connections(5)
            .connect_timeout(std::time::Duration::from_secs(30))
            .idle_timeout(std::time::Duration::from_secs(600))
            .max_lifetime(std::time::Duration::from_secs(1800))
            .connect(connection_string)
            .await?;

        info!("Database connected successfully");
        Ok(Self { pool })
    }

    /// Get the underlying pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get mutable pool reference
    pub fn pool_mut(&mut self) -> &mut PgPool {
        &mut self.pool
    }

    /// Run migrations
    pub async fn run_migrations(&self, migrations_path: &Path) -> Result<(), DbError> {
        info!("Running database migrations...");

        let m = Migrator::new(migrations_path).await?;
        m.run(&self.pool).await?;

        info!("Migrations completed successfully");
        Ok(())
    }

    /// Get connection for raw queries
    pub async fn acquire(&self) -> Result<sqlx::pool::PoolConnection<'_, sqlx::Postgres>, DbError> {
        Ok(self.pool.acquire().await?)
    }

    /// Begin a new transaction
    pub async fn begin(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, DbError> {
        Ok(self.pool.begin().await?)
    }
}

/// User model for database operations
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct UserModel {
    pub id: uuid::Uuid,
    pub email: String,
    pub phone: String,
    pub password_hash: String,
    pub role: String,
    pub status: String,
    pub verification_level: i32,
    pub mfa_enabled: bool,
    pub mfa_secret: Option<String>,
    pub last_login_at: Option<chrono::DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

/// User profile model
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct UserProfileModel {
    pub user_id: uuid::Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub gender: Option<String>,
    pub address: Option<serde_json::Value>,
    pub business_name: Option<String>,
    pub business_registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

/// Verification record model
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct VerificationRecordModel {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub level: i32,
    pub status: String,
    pub method: String,
    pub document_type: Option<String>,
    pub document_url: Option<String>,
    pub document_hash: Option<String>,
    pub verified_by: Option<uuid::Uuid>,
    pub verified_at: Option<chrono::DateTime<Utc>>,
    pub expires_at: Option<chrono::DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: chrono::DateTime<Utc>,
}

/// Session model
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct SessionModel {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub device_id: String,
    pub user_agent: String,
    pub ip_address: String,
    pub token_hash: String,
    pub refresh_token_hash: String,
    pub expires_at: chrono::DateTime<Utc>,
    pub refresh_expires_at: chrono::DateTime<Utc>,
    pub revoked: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub last_activity_at: chrono::DateTime<Utc>,
}

/// Role model
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct RoleModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub permissions: serde_json::Value,
    pub scope: String,
    pub role_level: i32,
    pub is_active: bool,
    pub is_system_role: bool,
    pub is_default: bool,
    pub category: Option<String>,
    pub parent_role_id: Option<uuid::Uuid>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

/// Permission model
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct PermissionModel {
    pub id: uuid::Uuid,
    pub role_id: uuid::Uuid,
    pub resource: String,
    pub action: String,
    pub conditions: Option<serde_json::Value>,
    pub fields: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<Utc>,
}

/// Invite code model
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct InviteCodeModel {
    pub code: String,
    pub max_uses: i32,
    pub current_uses: i32,
    pub expires_at: Option<chrono::DateTime<Utc>>,
    pub created_by: uuid::Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

/// Rate limit model
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RateLimitModel {
    pub key: String,
    pub count: i64,
    pub expires_at: chrono::DateTime<Utc>,
}

impl From<UserModel> for crate::domain::entities::User {
    fn from(model: UserModel) -> Self {
        Self {
            id: crate::domain::entities::UserId(model.id),
            email: crate::domain::entities::EmailAddress(model.email),
            phone: crate::domain::entities::PhoneNumber(model.phone),
            password_hash: crate::domain::entities::PasswordHash(model.password_hash),
            role: model
                .role
                .parse()
                .unwrap_or(crate::domain::enums::UserRole::Buyer),
            status: model
                .status
                .parse()
                .unwrap_or(crate::domain::enums::UserStatus::Pending),
            verification_level: match model.verification_level {
                0 => crate::domain::enums::VerificationLevel::Level0,
                1 => crate::domain::enums::VerificationLevel::Level1,
                2 => crate::domain::enums::VerificationLevel::Level2,
                3 => crate::domain::enums::VerificationLevel::Level3,
                4 => crate::domain::enums::VerificationLevel::Level4,
                _ => crate::domain::enums::VerificationLevel::Level0,
            },
            mfa_enabled: model.mfa_enabled,
            mfa_secret: model.mfa_secret.map(crate::domain::entities::Secret),
            last_login_at: model.last_login_at,
            metadata: crate::domain::entities::Metadata(model.metadata),
            created_at: model.created_at,
            updated_at: model.updated_at,
            deleted_at: model.deleted_at,
        }
    }
}

impl From<&crate::domain::entities::User> for UserModel {
    fn from(user: &crate::domain::entities::User) -> Self {
        Self {
            id: user.id.0,
            email: user.email.0.clone(),
            phone: user.phone.0.clone(),
            password_hash: user.password_hash.0.clone(),
            role: format!("{:?}", user.role),
            status: format!("{:?}", user.status),
            verification_level: user.verification_level as i32,
            mfa_enabled: user.mfa_enabled,
            mfa_secret: user.mfa_secret.as_ref().map(|s| s.0.clone()),
            last_login_at: user.last_login_at,
            metadata: user.metadata.0.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_model_conversion() {
        let model = UserModel {
            id: uuid::Uuid::new_v4(),
            email: "test@example.com".to_string(),
            phone: "+2348012345678".to_string(),
            password_hash: "hash123".to_string(),
            role: "BUYER".to_string(),
            status: "ACTIVE".to_string(),
            verification_level: 2,
            mfa_enabled: false,
            mfa_secret: None,
            last_login_at: None,
            metadata: serde_json::json!({}),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        };

        let user: crate::domain::entities::User = model.into();
        assert_eq!(user.email.0, "test@example.com");
        assert_eq!(user.phone.0, "+2348012345678");
    }
}
