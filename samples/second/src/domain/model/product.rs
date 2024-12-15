use crate::domain::user_context::UserContext;

#[derive(Clone,Debug)]
pub struct Product {
    product_id: String,
    price: f64,
}

impl Product {
    pub fn apply_discount(&mut self,user_context: &dyn UserContext) {
        if user_context.is_role(super::role::Role::Preffered) {
            self.price *=  0.95
        }
    }
    pub fn id(&self) -> &str {
        &self.product_id
    }
}