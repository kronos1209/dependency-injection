use std::sync::Arc;

use crate::domain::model::user::User;

#[async_trait::async_trait]
pub trait UserService: Sync + Send {
    async fn get_user(&self, user_id: String) -> anyhow::Result<Option<User>>;
    async fn create_user(&self, name: String) -> anyhow::Result<User>;
    async fn delete_user(&self, user_id: String) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl<T> UserService for Arc<T>
where
    T: UserService,
{
    async fn get_user(&self, user_id: String) -> anyhow::Result<Option<User>> {
        <T as UserService>::get_user(self, user_id).await
    }
    async fn create_user(&self, name: String) -> anyhow::Result<User> {
        <T as UserService>::create_user(self, name).await
    }
    async fn delete_user(&self, user_id: String) -> anyhow::Result<()> {
        <T as UserService>::delete_user(self, user_id).await
    }
}
