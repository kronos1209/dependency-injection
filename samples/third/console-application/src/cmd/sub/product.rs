use core::clone::Clone;

use crate::{
    action::{add_product::AddProductAction, Action},
    args::product::create_product::CreateProductArgs,
    domain::service::product::ProductService,
};
#[derive(Debug, Clone, clap::Subcommand)]
pub enum ProductCmd {
    Create {
        #[command(flatten)]
        args: CreateProductArgs,
    },
}

pub trait ProductActionActivator<'a> {
    fn create(&self, product_service: &'a dyn ProductService) -> Box<dyn Action + 'a>;
}

impl<'a> ProductActionActivator<'a> for ProductCmd {
    fn create(&self, product_service: &'a dyn ProductService) -> Box<dyn Action + 'a> {
        match self {
            Self::Create { args } => Box::new(AddProductAction::new(args.clone(), product_service)),
        }
    }
}
