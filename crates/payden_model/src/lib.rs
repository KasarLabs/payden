use reactive_stores::Store;

#[derive(PartialEq, Default, Debug, Store)]
pub struct Model {
    balance: f64,
    address: String,
}

#[derive(PartialEq, Eq, Default, Debug, Clone)]
pub enum Page {
    #[default]
    Send,
    Receive,
    Faucet,
    Activity,
}
