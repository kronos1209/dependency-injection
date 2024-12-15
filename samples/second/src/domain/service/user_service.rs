use crate::domain::model::user::User;

#[async_trait::async_trait]
pub trait UserService {
    async fn get_user(&self,user_id: String) -> anyhow::Result<User>;
}