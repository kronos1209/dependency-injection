use crate::action::{help::Help, Action};
use clap::{ArgMatches, Command};

pub mod config;
pub mod sample;

/// ApplicationCLIBuilder
///
/// Application のコマンドをビルドするためのトレイト
///
/// ## サンプル
///
/// ```ignore
/// struct SampleApp
///
/// impl ApplicationCLIBuilder for SampleApp {
///     fn build() -> Command {
///         // ビルドメソッドの実装を上書きしたい場合はここで実装する
///     }
///     fn cli() -> Command {
///         // 以下は最低限の実装
///         // 必要であれば情報を追加する
///         clap::Command::new("sample")
///     }
///     fn sub_clis() -> impl IntoIterator<Item = Command> {
///         // SubCommand1, SubCommand2 は clap::CommandFactory トレイトを実装している
///         vec![
///             SubCommand1::command(),
///             SubCommand2::command(),
///         ]
///     }
/// }
/// ```
pub trait ApplicationCLIBuilder {
    // Self::{cli,sub_clis} をもとにアプリケーションのコマンドをビルドする
    fn build() -> Command {
        Self::cli().subcommands(Self::sub_clis())
    }
    /// アプリケーションのメインコマンドを定義する場所
    fn cli() -> Command;
    /// サブコマンドを定義する
    fn sub_clis() -> impl IntoIterator<Item = Command>;
}

pub trait CLIParser: ApplicationCLIBuilder {
    fn parse_arg(&mut self) -> anyhow::Result<Box<dyn Action + '_>>;
    fn action_map(
        &self,
        cmd_name: &str,
        matches: &ArgMatches,
    ) -> anyhow::Result<Box<dyn Action + '_>>;
    fn get_matches(&mut self) -> anyhow::Result<ArgMatches> {
        Ok(<Self as ApplicationCLIBuilder>::build().try_get_matches()?)
    }
    fn print_help() -> Help {
        Help::new(<Self as ApplicationCLIBuilder>::build())
    }
}
