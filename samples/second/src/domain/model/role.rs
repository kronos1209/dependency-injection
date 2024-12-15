pub enum Role {
    Preffered,
    NotPreffered,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match &self {
            Self::NotPreffered => "not_preffered".to_string(),
            Self::Preffered => "preffered".to_string()
        } 
    }
}