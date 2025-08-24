use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::DomainError;

use super::user::{NewUser, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, new_user: NewUser) -> Result<User, DomainError>;
    async fn get_user(&self, id: Uuid) -> Result<User, DomainError>;
}
