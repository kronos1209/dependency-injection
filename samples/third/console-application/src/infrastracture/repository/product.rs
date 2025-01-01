use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::domain::model::product::Product;
use crate::domain::repository::product::ProductRepository;

pub struct MemoryProductRepository {
    inner: Arc<Mutex<HashMap<String, Product>>>,
}

impl Default for MemoryProductRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryProductRepository {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::default())),
        }
    }
}

#[async_trait::async_trait]
impl ProductRepository for MemoryProductRepository {
    async fn get_product(
        &self,
        product_id: String,
    ) -> anyhow::Result<Option<crate::domain::model::product::Product>> {
        let opt_product = {
            let guard = self.inner.lock().await;
            guard.get(&product_id).cloned()
        };
        Ok(opt_product)
    }
    async fn create_product(&self, name: String, price: f64) -> anyhow::Result<Product> {
        let product = Product::new(name, price);
        {
            let mut guard = self.inner.lock().await;
            if guard.contains_key(product.id()) {
                anyhow::bail!("already exist user [user_id: {:?}]", product.id())
            };
            guard.insert(product.id().clone(), product.clone());
        }
        Ok(product)
    }
    async fn delete_product(&self, product_id: String) -> anyhow::Result<()> {
        {
            let mut guard = self.inner.lock().await;
            if guard.remove(&product_id).is_none() {
                anyhow::bail!("not found product [product_id: {:?}]", &product_id)
            };
        }
        Ok(())
    }
}
