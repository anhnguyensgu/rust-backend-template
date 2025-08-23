use uuid::Uuid;

use super::errors::DomainError;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

impl NewUser {
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.name.trim().is_empty() {
            return Err(DomainError::Validation("name is required".into()));
        }
        if self.email.trim().is_empty() {
            return Err(DomainError::Validation("email is required".into()));
        }
        if !self.email.contains('@') {
            return Err(DomainError::Validation("email is invalid".into()));
        }
        Ok(())
    }
}

