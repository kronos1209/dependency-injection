use clap::{Command, CommandFactory, Parser};

pub mod add_product;
pub mod add_user;
/// clap の derive マクロによって生成された clap::Command に対して、
/// コマンド名を明示的に保持できるように拡張したビルダートレイト
/// ```ignore
/// #[derive(clap::Parser)]
/// struct SampleCmd {
///     args: SampleArg
/// }
///
/// impl CommandBuilder for SampleCmd {
///     const NAME: &'static str = "sample"
/// }
///
/// // CommandFactory::command() にも該当するして曖昧となるため、
/// // <SOME_CMD as CommandBuilder>::command() 明示的に指定する必要がある。
/// let cmd = <SampleCmd as CommandBuilder>::command();
/// ```
pub trait CommandBuilder: Parser {
    // コマンド名
    const NAME: &'static str;
    fn name() -> &'static str {
        Self::NAME
    }
    fn command() -> Command {
        <Self as CommandFactory>::command_for_update().name(Self::NAME)
    }
}

/// Command から Action に渡す必要がある情報を取得できる機能を提供する
pub trait CommandArgExt {
    type Args;
    fn args(&self) -> Self::Args;
}
