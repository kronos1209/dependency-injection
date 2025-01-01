use super::sub::SubCmd;

#[derive(clap::Parser, Debug)]
pub struct RootCmd {
    #[command(subcommand)]
    subcommand: SubCmd,
}

impl RootCmd {
    pub fn get_subcommand(&self) -> &SubCmd {
        &self.subcommand
    }
}
