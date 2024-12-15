use std::sync::Arc;

use crate::{domain::service::{product_service::ProductService, user_service::UserService}, infrastructure::{repository::{product_repository::MemeoryProductRepository, user_repository::MemoryUserRepositoryImpl}, service::{product_service::ProductServiceImpl, user_service::UserServiceImpl}}};

use super::config::AppConfig;

#[derive(Clone)]
pub struct AppModule {
    user_service: Arc<dyn UserService + Send + Sync>,
    product_service: Arc<dyn ProductService + Send + Sync>,
}

impl AppModule {
    pub fn from_config(_config: &AppConfig) -> Self {
        Self {
            user_service: Arc::new(UserServiceImpl::new(
                MemoryUserRepositoryImpl::new()
            )),
            product_service:Arc::new(ProductServiceImpl::new(
                MemeoryProductRepository::new()
            ))
        }
    }
    pub fn user_service<'this>(&'this self) -> &'this dyn UserService {
        self.user_service.as_ref()
    } 
    pub fn product_service<'this>(&'this self) -> &'this dyn ProductService {
        self.product_service.as_ref()
    }
}

