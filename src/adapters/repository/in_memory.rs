use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::{self, errors::DomainError, user::User};

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
impl crate::domain::ports::UserRepository for InMemoryUserRepo {
    async fn create(&self, new_user: crate::domain::user::NewUser) -> Result<User, DomainError> {
        let user = User {
            id: Uuid::new_v4(),
            name: new_user.name,
            email: new_user.email,
        };
        let mut write = self.store.write().await;
        write.insert(user.id, user.clone());
        Ok(user)
    }

    async fn get_user(&self, u: Uuid) -> Result<User, DomainError> {
        let read = self.store.read().await;
        read
            .get(&u)
            .cloned()
            .ok_or(domain::errors::DomainError::NotFound)
    }
}
