use uuid::Uuid;

use crate::domain::{
    errors::DomainError,
    ports::UserRepository,
    user::{NewUser, User},
};

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<User, DomainError> {
        new_user.validate()?;
        self.repo.create(new_user).await
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, DomainError> {
        self.repo.get(id).await
    }
}
