use super::Action;
use crate::args::add_user::AddUserArgs;
use crate::cmd::add_user::AddUserCmd;
use crate::cmd::CommandArgExt;
use crate::domain::service::user::UserService;

pub struct AddUserAction<'this> {
    args: AddUserArgs,
    user_service: &'this dyn UserService,
}

impl<'this> AddUserAction<'this> {
    pub fn new(args: AddUserArgs, user_service: &'this dyn UserService) -> Self {
        Self { args, user_service }
    }
}

impl AddUserCmd {
    pub fn action<'this>(&self, user_service: &'this dyn UserService) -> AddUserAction<'this> {
        AddUserAction::new(self.args(), user_service)
    }
}

#[async_trait::async_trait]
impl<'this> Action for AddUserAction<'this> {
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
