use actix_web::http::header::HeaderMap;

use crate::domain::{model::role::Role, user_context::UserContext};

pub struct HttpUserContextAdapter(actix_web::http::header::HeaderMap);

impl HttpUserContextAdapter {
    pub fn new(header: HeaderMap) -> Self {
        Self(header)
    }
}

const ROLE_KEY: &'static str = "user_role";
impl UserContext for HttpUserContextAdapter {
    fn is_role(&self,role: Role) -> bool {
        let Some(value) = self.0.get(ROLE_KEY) else {
            return false
        };
        return role.to_string() == *value
    }
}