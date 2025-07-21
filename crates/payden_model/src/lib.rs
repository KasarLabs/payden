use reactive_stores::Store;

#[derive(PartialEq, Default, Debug, Store)]
pub struct Model {
    page: Page,
    address: String,
    balance: f64,
    amount_send: String,
    amount_faucet: String,
}

#[derive(PartialEq, Eq, Default, Debug, Clone)]
pub enum Page {
    #[default]
    Send,
    Receive,
    Faucet,
    Activity,
}
