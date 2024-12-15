pub mod service {
    pub mod user_service;
    pub mod product_service;
}

pub mod model {
    pub mod user;
    pub mod product;
    pub mod role;
}

pub mod repository {
    pub mod user_repository;
    pub mod product_repository;
}

pub mod user_context;