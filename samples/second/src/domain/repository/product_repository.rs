use crate::domain::{model::{product::Product, user::User}, user_context::UserContext};

#[async_trait::async_trait]
pub trait ProductRepository {
    async fn find_by_id(&self,product_id: String) -> anyhow::Result<Option<Product>>;
    async fn save(&self,product: Product) -> anyhow::Result<()>;
    async fn delete(&self,product_id: String) -> anyhow::Result<()>;
}