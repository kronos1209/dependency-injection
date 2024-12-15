use super::model::role::Role;

pub trait UserContext {
    fn is_role(&self,role: Role) -> bool;
}