use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::{
    errors::DomainError,
    ports::UserRepository,
    user::{NewUser, User},
};

#[derive(Default, Clone)]
pub struct InMemoryUserRepo {
    store: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl InMemoryUserRepo {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepo {
    async fn create(&self, new_user: NewUser) -> Result<User, DomainError> {
        let mut write = self.store.write().await;
        // ensure email uniqueness (basic check)
        if write
            .values()
            .any(|u| u.email.eq_ignore_ascii_case(&new_user.email))
        {
            return Err(DomainError::Conflict("email already exists".into()));
        }

        let user = User {
            id: Uuid::new_v4(),
            name: new_user.name,
            email: new_user.email,
        };
        write.insert(user.id, user.clone());
        Ok(user)
    }

    async fn get(&self, id: Uuid) -> Result<User, DomainError> {
        let read = self.store.read().await;
        read.get(&id).cloned().ok_or(DomainError::NotFound)
    }
}

