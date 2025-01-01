use clap::Args;
#[derive(Clone, Args, Debug)]
pub struct CreateProductArgs {
    // ここに必要な引数を追加する
    name: String,
    price: f64,
}

impl CreateProductArgs {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn price(&self) -> f64 {
        self.price
    }
}
