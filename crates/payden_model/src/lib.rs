use reactive_stores::Store;

#[derive(PartialEq, Default, Debug, Store)]
pub struct Model {
    page: Page,
    addres: String,
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
