use crate::action::Action;
use clap::Parser;

pub mod config;
pub mod sample;

pub trait CLIParser {
    type RootCommand: Parser;
    fn parse_arg() -> Self::RootCommand {
        <Self::RootCommand as Parser>::parse()
    }
    // fn create_action(&self) -> Box<dyn Action + '_>;
}

pub trait ActionActivator<'a>: CLIParser {
    fn create(&'a self, cmd: Self::RootCommand) -> Box<dyn Action + '_>;
}
