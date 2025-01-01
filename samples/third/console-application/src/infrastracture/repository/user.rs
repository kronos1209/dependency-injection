use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::domain::{model::user::User, repository::user::UserRepository};

pub struct MemoryUserRepository {
    inner: Arc<Mutex<HashMap<String, User>>>,
}

impl Default for MemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryUserRepository {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::default())),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for MemoryUserRepository {
    async fn get_user(
        &self,
        user_id: String,
    ) -> anyhow::Result<Option<crate::domain::model::user::User>> {
        let opt_user = {
            let guard = self.inner.lock().await;
            guard.get(&user_id).cloned()
        };
        Ok(opt_user)
    }
    async fn create_user(&self, name: String) -> anyhow::Result<User> {
        let user = User::new(name);
        {
            let mut guard = self.inner.lock().await;
            if guard.contains_key(user.id()) {
                anyhow::bail!("already exist user [user_id: {:?}]", user.id())
            };
            guard.insert(user.id().clone(), user.clone());
        }
        Ok(user)
    }
    async fn delete_user(&self, user_id: String) -> anyhow::Result<()> {
        {
            let mut guard = self.inner.lock().await;
            if guard.remove(&user_id).is_none() {
                anyhow::bail!("not found user [user_id: {:?}]", &user_id)
            };
        }
        Ok(())
    }
}
