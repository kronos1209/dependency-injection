use std::collections::HashMap;

use crate::domain::{model::user::User, repository::user_repository::UserRepository};

pub struct MemoryUserRepositoryImpl {
    inner: tokio::sync::RwLock<HashMap<String,User>>
}

impl MemoryUserRepositoryImpl {
    pub fn new() -> Self {
        Self {
            inner: tokio::sync::RwLock::new(HashMap::default())
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for  MemoryUserRepositoryImpl
{
    async fn find_by_id(&self, user_id: String) -> anyhow::Result<Option<User>> {
        let user = {
            let guard = self.inner.read().await;
            guard.get(&user_id).cloned()
        };
        Ok(user)
    }
    async fn save(&self,user: User) -> anyhow::Result<()> {
        {
            let mut guard = self.inner.write().await;
            guard.insert(user.id().to_string(), user);
        }
        Ok(())
    }

    async fn delete(&self,user_id: String) -> anyhow::Result<()> {
        {
            let mut guard = self.inner.write().await;
            guard.remove(&user_id);
        }
        Ok(())
    }
}
    