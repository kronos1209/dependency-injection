use clap::Args;
#[derive(Clone, Args, Debug)]
pub struct CreateUserArgs {
    name: String,
}

impl CreateUserArgs {
    pub fn name(&self) -> &String {
        &self.name
    }
}
