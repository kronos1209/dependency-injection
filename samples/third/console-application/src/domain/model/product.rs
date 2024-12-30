use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    id: String,
    name: String,
    price: f64,
}

impl Product {
    pub fn new(name: String, price: f64) -> Self {
        Self {
            id: Uuid::default().to_string(),
            name,
            price,
        }
    }
    pub fn id(&self) -> &String {
        &self.id
    }
}
