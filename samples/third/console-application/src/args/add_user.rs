use clap::Args;
#[derive(Clone, Args, Debug)]
pub struct AddUserArgs {
    name: String,
}

impl AddUserArgs {
    pub fn name(&self) -> &String {
        &self.name
    }
}
