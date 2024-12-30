use std::sync::Arc;

use clap::{ArgMatches, Command, FromArgMatches};

use crate::{
    action::Action,
    cmd::{add_product::AddProductCmd, add_user::AddUserCmd, CommandBuilder},
    domain::service::{product::ProductService, user::UserService},
    infrastracture::{
        repository::{product::MemoryProductRepository, user::MemoryUserRepository},
        service::{product::ProductServiceImpl, user::UserServiceImpl},
    },
};

use super::{config::AppConfig, ApplicationCLIBuilder, CLIParser};

pub struct SampleApplication {
    service_container: ServficeContiner,
}

struct ServficeContiner {
    user_service: Arc<dyn UserService + Send + Sync>,
    product_service: Arc<dyn ProductService + Send + Sync>,
}
impl SampleApplication {
    pub fn new(config: AppConfig) -> Self {
        Self {
            service_container: Self::create_object_map(config),
        }
    }
    pub async fn execute(&mut self) -> anyhow::Result<()> {
        let action = self.parse_arg()?;
        action.execute().await
    }
    fn create_object_map(_config: AppConfig) -> ServficeContiner {
        ServficeContiner {
            user_service: Arc::new(UserServiceImpl::new(MemoryUserRepository::new())),
            product_service: Arc::new(ProductServiceImpl::new(MemoryProductRepository::new())),
        }
    }
}

impl ApplicationCLIBuilder for SampleApplication {
    fn cli() -> clap::Command {
        Command::new("sample").about("sample application cli")
    }
    fn sub_clis() -> impl IntoIterator<Item = Command> {
        vec![
            <AddUserCmd as CommandBuilder>::command(),
            <AddProductCmd as CommandBuilder>::command(),
        ]
    }
}

impl CLIParser for SampleApplication {
    fn parse_arg(&mut self) -> anyhow::Result<Box<dyn crate::action::Action + '_>> {
        let matches = self.get_matches()?;

        // subcommand が指定されていない場合は help を表示する action を返す
        let Some((cmd_name, sub_matches)) = matches.subcommand() else {
            return Ok(Box::new(<Self as CLIParser>::print_help()));
        };

        // コマンド名と引数から Action トレイトオブジェクトを返す
        let action = self.action_map(cmd_name, sub_matches)?;
        Ok(action)
    }
    fn action_map(
        &self,
        cmd_name: &str,
        matches: &ArgMatches,
    ) -> anyhow::Result<Box<dyn Action + '_>> {
        let action: Box<dyn Action> = match cmd_name {
            AddUserCmd::NAME => Box::new(
                <AddUserCmd as FromArgMatches>::from_arg_matches(matches)?
                    .action(self.service_container.user_service.as_ref()),
            ),
            AddProductCmd::NAME => Box::new(
                <AddProductCmd as FromArgMatches>::from_arg_matches(matches)?
                    .action(self.service_container.product_service.as_ref()),
            ),
            _ => anyhow::bail!("unknow subcommand: {:?}", cmd_name),
        };
        Ok(action)
    }
}
