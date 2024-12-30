use uuid::Uuid;

#[derive(Clone, PartialEq, Debug)]
pub struct User {
    id: String,
    name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::default().to_string(),
            name,
        }
    }
    pub fn id(&self) -> &String {
        &self.id
    }
}
