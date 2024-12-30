use crate::domain::model::product::Product;

#[async_trait::async_trait]
pub trait ProductRepository: Send + Sync {
    async fn get_product(&self, product_id: String) -> anyhow::Result<Option<Product>>;
    async fn create_product(&self, name: String, price: f64) -> anyhow::Result<Product>;
    async fn delete_product(&self, product_id: String) -> anyhow::Result<()>;
}
