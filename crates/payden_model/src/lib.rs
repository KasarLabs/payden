use reactive_stores::Store;

#[derive(PartialEq, Default, Debug, Store)]
pub struct Model {
    page: Page,
    address: String,
    send_amount: String,
}

#[derive(PartialEq, Eq, Default, Debug, Clone)]
pub enum Page {
    #[default]
    Send,
    Receive,
    Faucet,
    Activity,
}
