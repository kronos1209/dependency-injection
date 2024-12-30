use crate::action::Action;
use clap::Command;

pub struct Help {
    cmd: Command,
}
impl Help {
    pub fn new(cmd: Command) -> Self {
        Self { cmd }
    }
}
#[async_trait::async_trait]
impl Action for Help {
    async fn execute(&self) -> anyhow::Result<()> {
        self.cmd.clone().print_long_help()?;
        Ok(())
    }
}
