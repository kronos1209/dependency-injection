use crate::domain::{model::product::Product, user_context::UserContext};

#[async_trait::async_trait]
pub trait ProductService {
    async fn get_product(&self,product_id: String, user_context: &(dyn UserContext + Sync + Send)) -> anyhow::Result<Product>;
}