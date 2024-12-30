use std::sync::Arc;

use crate::domain::model::product::Product;

#[async_trait::async_trait]
pub trait ProductService: Send + Sync {
    async fn get_product(&self, product_id: String) -> anyhow::Result<Option<Product>>;
    async fn create_product(&self, name: String, price: f64) -> anyhow::Result<Product>;
    async fn delete_product(&self, product_id: String) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl<T> ProductService for Arc<T>
where
    T: ProductService,
{
    async fn get_product(&self, product_id: String) -> anyhow::Result<Option<Product>> {
        <T as ProductService>::get_product(self, product_id).await
    }
    async fn create_product(&self, name: String, price: f64) -> anyhow::Result<Product> {
        <T as ProductService>::create_product(self, name, price).await
    }
    async fn delete_product(&self, product_id: String) -> anyhow::Result<()> {
        <T as ProductService>::delete_product(self, product_id).await
    }
}
