use super::Action;
use crate::args::user::create_user::CreateUserArgs;
use crate::domain::service::user::UserService;

pub struct CreateUserAction<'this> {
    args: CreateUserArgs,
    user_service: &'this dyn UserService,
}

impl<'this> CreateUserAction<'this> {
    pub fn new(args: CreateUserArgs, user_service: &'this dyn UserService) -> Self {
        Self { args, user_service }
    }
}

#[async_trait::async_trait]
impl<'this> Action for CreateUserAction<'this>
{
    async fn execute(&self) -> anyhow::Result<()> {
        let user = self
            .user_service
            .create_user(self.args.name().to_string())
            .await?;

        // test 用に作成した product を再度 service 経由で取得する
        if let Ok(Some(re)) = self.user_service.get_user(user.id().clone()).await {
            println!("{:?}", re);
            assert_eq!(user, re);
        } else {
            panic!("failed to add product")
        };
        Ok(())
    }
}
