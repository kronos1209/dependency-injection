use crate::domain::{model::product::Product, repository::product_repository::ProductRepository, service::product_service::ProductService, user_context::UserContext};

pub struct ProductServiceImpl<R> {
    product_repository: R
}

impl<R> ProductServiceImpl<R> {
    pub fn new(product_repository:R) -> Self {
        Self {
            product_repository: product_repository
        }
    }
}

#[async_trait::async_trait]
impl<R> ProductService for ProductServiceImpl<R> 
where 
    R: ProductRepository + Sync + Send
{
    async fn get_product(&self,product_id: String,user_context: &(dyn UserContext + Sync + Send)) -> anyhow::Result<Product> {
        let Some(mut product) = self.product_repository.find_by_id(product_id).await? else {
            anyhow::bail!("not found product");
        };

        product.apply_discount(user_context);
        Ok(product)
    }
}