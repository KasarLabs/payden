use reactive_stores::Store;

#[derive(PartialEq, Eq, Default, Debug, Store)]
pub struct Model {
    page: Page,
}

#[derive(PartialEq, Eq, Default, Debug, Clone)]
pub enum Page {
    #[default]
    Send,
    Receive,
    Faucet,
    Activity,
}
