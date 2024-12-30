use clap::Parser;

use crate::args::add_product::*;

use super::{CommandArgExt, CommandBuilder};

#[derive(Parser)]
pub struct AddProductCmd {
    #[command(flatten)]
    args: AddProductArgs,
}

impl CommandBuilder for AddProductCmd {
    const NAME: &'static str = "addproduct";
}

impl CommandArgExt for AddProductCmd {
    type Args = AddProductArgs;
    fn args(&self) -> Self::Args {
        self.args.clone()
    }
}
