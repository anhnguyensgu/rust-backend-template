use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::user::User;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Default)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}

