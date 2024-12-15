use crate::domain::{model::user, repository::user_repository::UserRepository, service::user_service::UserService};

pub struct UserServiceImpl<R> {
    user_repository: R
}

impl<R> UserServiceImpl<R> {
    pub fn new(user_repository: R) -> Self {
        Self {
            user_repository: user_repository
        }
    } 
}

#[async_trait::async_trait]
impl<R> UserService for UserServiceImpl<R>
where 
    R : UserRepository + Sync + Send
{
    async fn get_user(&self,user_id: String) -> anyhow::Result<user::User> {
        let Some(user) = self.user_repository.find_by_id(user_id.clone()).await? else {
            anyhow::bail!("not foud user [user_id => {:?}",&user_id);
        };

        return Ok(user)
    }
}