use crate::domain::{
    model::user::User, repository::user::UserRepository, service::user::UserService,
};

pub struct UserServiceImpl<T> {
    user_repository: T,
}

impl<T> UserServiceImpl<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl<T> UserService for UserServiceImpl<T>
where
    T: UserRepository,
{
    async fn get_user(
        &self,
        user_id: String,
    ) -> anyhow::Result<Option<crate::domain::model::user::User>> {
        self.user_repository.get_user(user_id).await
    }
    async fn create_user(&self, name: String) -> anyhow::Result<User> {
        self.user_repository.create_user(name).await
    }
    async fn delete_user(&self, user_id: String) -> anyhow::Result<()> {
        self.user_repository.delete_user(user_id).await
    }
}
