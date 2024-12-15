use crate::domain::model::user::User;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, user_id: String) -> anyhow::Result<Option<User>>;
    async fn save(&self, user: User) -> anyhow::Result<()>;
    async fn delete(&self, user_id: String) -> anyhow::Result<()>;
}