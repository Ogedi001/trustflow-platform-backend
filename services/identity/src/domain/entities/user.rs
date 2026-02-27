use crate::domain::{
    entities::RoleId,
    enums::{UserRole, UserStatus, VerificationLevel},
};
use common::{EmailAddress, PasswordHash, PhoneNumber, Timestamp};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: EmailAddress,
    pub phone: PhoneNumber,
    pub password_hash: PasswordHash,
    pub role: RoleId,
    pub status: UserStatus,
    pub verification_level: VerificationLevel,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted_at: Option<Timestamp>,
}

impl User {
    pub fn new_pending(email: EmailAddress, password_hash: PasswordHash, role: RoleId) -> Self {
        Self {
            id: UserId::new(),
            email,
            phone: None,
            password_hash,
            role,
            status: UserStatus::Pending,
            verification_level: VerificationLevel::Level0,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
            deleted_at: None,
        }
    }

    pub fn activate(&mut self) {
        self.status = UserStatus::Active;
        self.updated_at = Timestamp::now();
    }

    /// Suspend user with reason
    pub fn suspend(&mut self, reason: &str) {
        self.status = UserStatus::Suspended;
        self.metadata.insert("suspension_reason", reason);
        self.updated_at = Timestamp::now();
    }

    /// Update last login timestamp
    pub fn update_last_login(&mut self) {
        self.last_login_at = Some(Timestamp::now());
        self.updated_at = Timestamp::now();
    }

    /// Check if user is active
    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }

    /// Check if user is deleted
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Change user role
    pub fn change_role(&mut self, new_role: RoleId) {
        self.role = new_role;
        self.updated_at = Timestamp::now();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]

/// User ID newtype wrapper for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// User profile entity - extended user information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: UserId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: String,
    pub avatar_url: Option<Url>,
    pub date_of_birth: Option<DateWrapper>,
    pub gender: Option<Gender>,
    pub address: Option<Address>,
    pub business_name: Option<String>,
    pub business_registration_number: Option<String>, // CAC number for Nigeria
    pub tax_id: Option<String>,                       // Nigerian TIN
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl UserProfile {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            first_name: None,
            last_name: None,
            display_name: String::new(),
            avatar_url: None,
            date_of_birth: None,
            gender: None,
            address: None,
            business_name: None,
            business_registration_number: None,
            tax_id: None,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
        }
    }

    /// Get full name if available
    pub fn full_name(&self) -> Option<String> {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            (Some(first), None) => Some(first.clone()),
            (None, Some(last)) => Some(last.clone()),
            _ => None,
        }
    }

    /// Check if profile is complete for business verification
    pub fn is_business_complete(&self) -> bool {
        self.business_name.is_some() && self.business_registration_number.is_some()
    }

    /// Check if profile is complete for personal verification
    pub fn is_personal_complete(&self) -> bool {
        self.first_name.is_some() && self.last_name.is_some() && self.date_of_birth.is_some()
    }
}

/// Gender enum for user profile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    Other,
}

/// Address value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: Option<String>,
}

/// Date wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateWrapper(pub time::Date);
