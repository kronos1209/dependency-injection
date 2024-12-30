use crate::domain::model::user::User;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(&self, user_id: String) -> anyhow::Result<Option<User>>;
    async fn create_user(&self, name: String) -> anyhow::Result<User>;
    async fn delete_user(&self, user_id: String) -> anyhow::Result<()>;
}
