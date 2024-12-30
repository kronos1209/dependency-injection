use crate::{
    args::add_product::*, cmd::add_product::AddProductCmd, domain::service::product::ProductService,
};

use super::Action;
use crate::cmd::CommandArgExt;

pub struct AddProductAction<'this> {
    args: AddProductArgs,
    service: &'this dyn ProductService,
}

impl<'this> AddProductAction<'this> {
    pub fn new(args: AddProductArgs, product_service: &'this dyn ProductService) -> Self {
        Self {
            args,
            service: product_service,
        }
    }
}

impl AddProductCmd {
    pub fn action<'this>(
        &self,
        product_service: &'this dyn ProductService,
    ) -> AddProductAction<'this> {
        AddProductAction::new(self.args(), product_service)
    }
}

#[async_trait::async_trait]
impl<'this> Action for AddProductAction<'this> {
    async fn execute(&self) -> anyhow::Result<()> {
        let product = self
            .service
            .create_product(self.args.name().to_string(), self.args.price())
            .await?;

        // test 用に作成した product を再度 service 経由で取得する
        if let Ok(Some(re)) = self.service.get_product(product.id().clone()).await {
            println!("{:?}", re);
            assert_eq!(product, re);
        } else {
            panic!("failed to add product")
        };
        Ok(())
    }
}
