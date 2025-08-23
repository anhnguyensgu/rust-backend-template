use async_trait::async_trait;
use uuid::Uuid;

use super::{errors::DomainError, user::{NewUser, User}};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, new_user: NewUser) -> Result<User, DomainError>;
    async fn get(&self, id: Uuid) -> Result<User, DomainError>;
}

