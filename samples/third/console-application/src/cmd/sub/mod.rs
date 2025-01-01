use product::ProductCmd;
use user::UserCmd;

pub mod product;
pub mod user;

#[derive(Debug,clap::Subcommand,Clone)]
pub enum SubCmd {
    #[command(subcommand)]
    User(UserCmd),
    #[command(subcommand)]
    Product(ProductCmd)
}