#[derive(Clone,Debug)]
pub struct User {
    user_id: String,
    name: String,
}
impl User {
    pub fn new(name: String) -> Self {
        Self {
            user_id: "hogehoe".to_string(),
            name: name,
        }
    }
    pub fn id(&self) -> &str {
        &self.user_id
    }
}
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}