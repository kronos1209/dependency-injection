use std::sync::Arc;


use crate::{
    action::Action,
    cmd::{
        root::RootCmd,
        sub::{product::ProductActionActivator, user::UserActionActivator, SubCmd},
    },
    domain::service::{product::ProductService, user::UserService},
    infrastracture::{
        repository::{product::MemoryProductRepository, user::MemoryUserRepository},
        service::{product::ProductServiceImpl, user::UserServiceImpl},
    },
};

use super::{config::AppConfig, ActionActivator, CLIParser};

pub struct SampleApplication {
    service_container: ServficeContiner,
}

struct ServficeContiner {
    user_service: Arc<dyn UserService>,
    product_service: Arc<dyn ProductService>,
}
impl SampleApplication {
    pub fn new(config: AppConfig) -> Self {
        Self {
            service_container: Self::create_object_map(config),
        }
    }
    pub async fn execute(&self) -> anyhow::Result<()> {
        let cmd = <Self as CLIParser>::parse_arg();
        let action = self.create(cmd);
        action.execute().await
    }
    fn create_object_map(_config: AppConfig) -> ServficeContiner {
        ServficeContiner {
            user_service: Arc::new(UserServiceImpl::new(MemoryUserRepository::new())),
            product_service: Arc::new(ProductServiceImpl::new(MemoryProductRepository::new())),
        }
    }
}

impl CLIParser for SampleApplication {
    type RootCommand = RootCmd;
}

impl<'a> ActionActivator<'a> for SampleApplication {
    fn create(&'a self, cmd: Self::RootCommand) -> Box<dyn Action + '_> {
        let action: Box<dyn Action + '_> = match cmd.get_subcommand() {
            SubCmd::Product(product_cmd) => product_cmd.create(
                self.service_container.product_service.as_ref(),
            ),
            SubCmd::User(user_cmd) => user_cmd.create(self.service_container.user_service.as_ref()),
        };
        action
    }
}
