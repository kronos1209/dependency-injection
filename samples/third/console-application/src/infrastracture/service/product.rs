use crate::domain::{
    model::product::Product, repository::product::ProductRepository,
    service::product::ProductService,
};

pub struct ProductServiceImpl<T> {
    product_repository: T,
}

impl<T> ProductServiceImpl<T> {
    pub fn new(product_repository: T) -> Self {
        Self { product_repository }
    }
}

#[async_trait::async_trait]
impl<T> ProductService for ProductServiceImpl<T>
where
    T: ProductRepository,
{
    async fn get_product(
        &self,
        product_id: String,
    ) -> anyhow::Result<Option<crate::domain::model::product::Product>> {
        self.product_repository.get_product(product_id).await
    }
    async fn create_product(&self, name: String, price: f64) -> anyhow::Result<Product> {
        self.product_repository.create_product(name, price).await
    }
    async fn delete_product(&self, product_id: String) -> anyhow::Result<()> {
        self.product_repository.delete_product(product_id).await
    }
}
