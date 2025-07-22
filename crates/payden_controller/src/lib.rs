//! Based off the [Miden docs].
//!
//! [Miden docs]: https://0xmiden.github.io/miden-docs/imported/miden-tutorials/src/rust-client/create_deploy_tutorial.html

#![no_std]

extern crate alloc;

mod error;

pub struct Controller {
    client: miden_client::Client,
    model: reactive_stores::Store<payden_model::Model>,
}

impl Controller {
    pub async fn new() -> crate::error::Result<Self> {
        // Determine the number of blocks to consider a transaction stale.
        let tx_graceful_blocks = Some(20);

        // Determine the maximum number of blocks that the client can be behind from the network.
        let max_block_number_delta = Some(256);

        let client = miden_client::Client::new(
            Self::rpc_api(),
            Self::rng()?,
            Self::store().await?,
            Self::keystore()?,
            Self::execution()?,
            tx_graceful_blocks,
            max_block_number_delta,
        );

        let model = reactive_stores::Store::new(payden_model::Model::default());

        Ok(Self { client, model })
    }

    pub async fn regenerate_account(&mut self) {
        todo!()
    }

    pub async fn send(&mut self) {
        todo!()
    }

    pub async fn mint(&mut self) {
        todo!()
    }

    pub async fn sync(&mut self) {
        todo!()
    }

    fn rpc_api() -> alloc::sync::Arc<impl miden_client::rpc::NodeRpcClient + Send> {
        let endpoint = miden_client::rpc::Endpoint::testnet();
        let timeout_ms = 10_000;
        alloc::sync::Arc::new(miden_client::rpc::TonicRpcClient::new(&endpoint, timeout_ms))
    }

    fn rng() -> crate::error::Result<alloc::boxed::Box<dyn miden_client::crypto::FeltRng>> {
        let seed = [getrandom::u64()?, getrandom::u64()?, getrandom::u64()?, getrandom::u64()?];
        Ok(alloc::boxed::Box::new(miden_client::crypto::RpoRandomCoin::new(seed.map(miden_client::Felt::new))))
    }

    async fn store() -> crate::error::Result<alloc::sync::Arc<dyn miden_client::store::Store>> {
        Ok(alloc::sync::Arc::new(
            miden_client::store::web_store::WebStore::new().await.map_err(|_| crate::error::WebStoreInitError)?,
        ))
    }

    fn keystore() -> crate::error::Result<alloc::sync::Arc<dyn miden_client::auth::TransactionAuthenticator>> {
        let rng = Self::rng()?;
        Ok(alloc::sync::Arc::new(miden_client::keystore::WebKeyStore::new(rng)))
    }

    fn execution() -> crate::error::Result<miden_client::ExecutionOptions> {
        Ok(miden_client::ExecutionOptions::new(
            Some(miden_objects::MAX_TX_EXECUTION_CYCLES),
            miden_objects::MIN_TX_EXECUTION_CYCLES,
            false,
            false,
        )?)
    }
}
