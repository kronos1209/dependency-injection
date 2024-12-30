use clap::ArgMatches;
pub mod add_product;
pub mod add_user;
pub mod help;

pub trait ActionFactory {
    type Dependency;
    fn from_arg_matches(
        matches: &ArgMatches,
        dependency: Self::Dependency,
    ) -> anyhow::Result<Box<dyn Action>>;
    fn action(&self, dependency: Self::Dependency) -> Box<dyn Action>;
}

#[async_trait::async_trait]
pub trait Action {
    async fn execute(&self) -> anyhow::Result<()>;
}
