use core::clone::Clone;

use crate::{
    action::{add_user::CreateUserAction, Action},
    args::user::{create_user::CreateUserArgs},
    domain::service::user::UserService,
};

#[derive(clap::Subcommand, Debug, Clone)]
pub enum UserCmd {
    Create {
        #[command(flatten)]
        args: CreateUserArgs,
    },
}

/// Command 列挙体から Action を作成するトレイト
/// ここで Command 全体で必要な依存を定義しておく
/// もしコマンド追加することでここの依存を増やスことになった場合は
/// ドメイン境界が適切に分けられているか疑うこと
///
/// TODO: 実際はグローバルなコマンド引数が存在することが考えられるため、
/// 引数でグローバルコンテキストを抽象化したオブジェクトを受け取る
pub trait UserActionActivator<'a> {
    fn create(&self, user_service: &'a dyn UserService) -> Box<dyn Action + 'a>;
}

impl<'a> UserActionActivator<'a> for UserCmd {
    fn create(&self, user_service: &'a dyn UserService) -> Box<dyn Action + 'a> {
        match self {
            Self::Create { args } => Box::new(CreateUserAction::new(args.clone(), user_service)),
        }
    }
}
