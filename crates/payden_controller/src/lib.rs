//! Based off the [Miden docs].
//!
//! [Miden docs]: https://0xmiden.github.io/miden-docs/imported/miden-tutorials/src/rust-client/create_deploy_tutorial.html

#![cfg_attr(not(test), no_std)]

mod error;

use alloc::boxed::Box;
use alloc::sync::Arc;
use leptos::prelude::*;
use payden_model::*;
use rand_core::RngCore;

use crate::error::*;

extern crate alloc;

pub struct Controller {
    client: miden_client::Client,
    keystore: Arc<miden_client::keystore::WebKeyStore<Box<dyn miden_client::crypto::FeltRng>>>,
    pub model: reactive_stores::Store<payden_model::Model>,
}

impl Controller {
    // TODO: add back proper error handling here
    pub async fn new() -> Self {
        // Determine the number of blocks to consider a transaction stale.
        let tx_graceful_blocks = Some(20);

        // Determine the maximum number of blocks that the client can be behind from the network.
        let max_block_number_delta = Some(256);

        let keystore = Self::keystore().unwrap();

        let mut client = miden_client::Client::new(
            Self::rpc_api(),
            Self::rng().unwrap(),
            Self::store().await.unwrap(),
            keystore.clone(),
            Self::execution().unwrap(),
            tx_graceful_blocks,
            max_block_number_delta,
        );
        // TODO: display latest block number in UI
        let _sync_state = client.sync_state().await.unwrap();

        let model = reactive_stores::Store::new(payden_model::Model::default());

        Self { client, keystore, model }
    }

    pub async fn regenerate_account(&mut self) -> ResultDyn<()> {
        let mut account_seed = [0u8; 32];
        self.client.rng().fill_bytes(&mut account_seed);

        let keypair = miden_client::crypto::SecretKey::with_rng(self.client.rng());
        let public_key = keypair.public_key();

        let builder = miden_client::account::AccountBuilder::new(account_seed)
            .account_type(miden_client::account::AccountType::RegularAccountUpdatableCode)
            .storage_mode(miden_client::account::AccountStorageMode::Public)
            .with_component(miden_client::account::component::RpoFalcon512::new(public_key))
            .with_component(miden_client::account::component::BasicWallet);

        let (account, seed) = builder.build()?;

        self.client.add_account(&account, Some(seed), false /* overwrite */).await?;
        self.keystore.add_key(&miden_client::auth::AuthSecretKey::RpoFalcon512(keypair)).await?;

        let address = account.id().to_bech32(miden_objects::account::NetworkId::Testnet);
        self.model.address().set(address);

        Ok(())
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

    fn rpc_api() -> Arc<impl miden_client::rpc::NodeRpcClient + Send> {
        let endpoint = miden_client::rpc::Endpoint::testnet();
        let timeout_ms = 10_000;
        Arc::new(miden_client::rpc::TonicRpcClient::new(&endpoint, timeout_ms))
    }

    fn rng() -> ResultDyn<Box<dyn miden_client::crypto::FeltRng>> {
        let seed = [getrandom::u64()?, getrandom::u64()?, getrandom::u64()?, getrandom::u64()?];
        Ok(Box::new(miden_client::crypto::RpoRandomCoin::new(seed.map(miden_client::Felt::new))))
    }

    async fn store() -> ResultDyn<Arc<dyn miden_client::store::Store>> {
        Ok(Arc::new(miden_client::store::web_store::WebStore::new().await.map_err(|_| WebStoreInitError)?))
    }

    fn keystore() -> ResultDyn<Arc<miden_client::keystore::WebKeyStore<Box<dyn miden_client::crypto::FeltRng>>>> {
        let rng = Self::rng()?;
        Ok(Arc::new(miden_client::keystore::WebKeyStore::new(rng)))
    }

    fn execution() -> ResultDyn<miden_client::ExecutionOptions> {
        Ok(miden_client::ExecutionOptions::new(
            Some(miden_objects::MAX_TX_EXECUTION_CYCLES),
            miden_objects::MIN_TX_EXECUTION_CYCLES,
            false,
            false,
        )?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn controller() {
        let res = Controller::new().await;
        assert!(res.is_ok());
    }
}
