// Representation of a User model with role and status enums.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Possible roles for a user.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "user")]
    User,
}

/// Possible statuses for a user.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub enum Status {
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "active")]
    Active,
}

/// User model struct.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    /// Unique identifier for the user.
    pub id: Uuid,
    /// Role of the user.
    pub role: Role,
    /// Status of the user.
    pub status: Status,
    // Additional fields can be added as needed.
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub role: i32,
    pub status: i32,
    pub exp: u64,
}
