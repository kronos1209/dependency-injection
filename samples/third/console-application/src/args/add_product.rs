use clap::Args;
#[derive(Clone, Args, Debug)]
pub struct AddProductArgs {
    // ここに必要な引数を追加する
    name: String,
    price: f64,
}

impl AddProductArgs {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn price(&self) -> f64 {
        self.price
    }
}
