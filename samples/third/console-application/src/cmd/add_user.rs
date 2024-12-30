use clap::Parser;

use crate::args::add_user::AddUserArgs;

use super::{CommandArgExt, CommandBuilder};

#[derive(Parser)]
pub struct AddUserCmd {
    #[command(flatten)]
    args: AddUserArgs,
}

impl CommandBuilder for AddUserCmd {
    const NAME: &'static str = "adduser";
}

impl CommandArgExt for AddUserCmd {
    type Args = AddUserArgs;
    fn args(&self) -> Self::Args {
        self.args.clone()
    }
}
