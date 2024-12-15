use std::collections::HashMap;

use crate::domain::{model::product::Product, repository::product_repository::ProductRepository};

pub struct MemeoryProductRepository {
    inner: tokio::sync::RwLock<HashMap<String,Product>>
}

impl MemeoryProductRepository {
    pub fn new() -> Self {
        Self {
            inner: tokio::sync::RwLock::new(HashMap::default())
        }
    }
}

#[async_trait::async_trait]
impl ProductRepository for MemeoryProductRepository {
    async fn find_by_id(&self, product_id: String) -> anyhow::Result<Option<Product>> {
        let product = {
            let guard = self.inner.read().await;
            guard.get(&product_id).cloned()
        };
        Ok(product)
    }
    async fn save(&self,product: Product) -> anyhow::Result<()> {
        {
            let mut guard = self.inner.write().await;
            guard.insert(product.id().to_string(), product);
        }
        Ok(())
    }

    async fn delete(&self,product_id: String) -> anyhow::Result<()> {
        {
            let mut guard = self.inner.write().await;
            guard.remove(&product_id);
        }
        Ok(())
    }
}