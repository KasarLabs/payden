use reactive_stores::Store;

#[derive(PartialEq, Default, Debug, Store)]
pub struct Model {
    page: Page,
    balance: f64,
    address: String,
    address_send: String,
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
