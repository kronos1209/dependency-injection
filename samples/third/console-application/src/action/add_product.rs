use crate::{
    args::product::create_product::CreateProductArgs, domain::service::product::ProductService,
};

use super::Action;

pub struct AddProductAction<'this> {
    args: CreateProductArgs,
    service: &'this dyn ProductService,
}

impl<'this> AddProductAction<'this> {
    pub fn new(args: CreateProductArgs, product_service:&'this dyn ProductService) -> Self {
        Self {
            args,
            service: product_service,
        }
    }
}



#[async_trait::async_trait]
impl<'this> Action for AddProductAction<'this> 
{
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
