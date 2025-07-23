//! Based off the [Miden docs].
//!
//! [Miden docs]: https://0xmiden.github.io/miden-docs/imported/miden-tutorials/src/rust-client/create_deploy_tutorial.html

#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::string::ToString;

mod error;

use alloc::boxed::Box;
use alloc::sync::Arc;
use leptos::{logging, prelude::*};
use payden_model::*;
use rand_core::RngCore;

use crate::error::*;

#[derive(Clone, Copy)]
pub enum ControllerAction {
    Refresh,
    Send,
    Mint,
}

pub struct Controller {
    client: miden_client::Client,
    account_id: miden_objects::account::AccountId,
    store: Arc<miden_client::store::web_store::WebStore>,
    keystore: Arc<miden_client::keystore::WebKeyStore<Box<dyn miden_client::crypto::FeltRng>>>,
    pub model: reactive_stores::Store<payden_model::Model>,
}

impl Controller {
    pub async fn handle(&mut self, action: ControllerAction) {
        match action {
            ControllerAction::Refresh => {
                logging::log!("Refresh");
                self.account_sync().await.unwrap();
            }
            ControllerAction::Send => logging::log!("Sending"),
            ControllerAction::Mint => logging::log!("Minting"),
        }
    }
}

// TODO: add back proper error handling
impl Controller {
    pub async fn new() -> alloc::rc::Rc<core::cell::RefCell<Self>> {
        // Determine the number of blocks to consider a transaction stale.
        let tx_graceful_blocks = Some(20);

        // Determine the maximum number of blocks that the client can be behind from the network.
        let max_block_number_delta = Some(256);

        let store = Self::store().await.unwrap();
        let keystore = Self::keystore().unwrap();

        let mut client = miden_client::Client::new(
            Self::rpc_api(),
            Self::rng().unwrap(),
            store.clone(),
            keystore.clone(),
            Self::execution().unwrap(),
            tx_graceful_blocks,
            max_block_number_delta,
        );
        // TODO: display latest block number in UI
        let _sync_state = client.sync_state().await.unwrap();

        let model = reactive_stores::Store::new(payden_model::Model::default());

        // Restoring previous account
        let mut controller = if let Some((header, _)) = client.get_account_headers().await.unwrap().first() {
            Self::account_retrieve(client, store, keystore, model, header).unwrap()
        } else {
            Self::account_generate(client, store, keystore, model).await.unwrap()
        };

        alloc::rc::Rc::new(controller.into())
    }

    pub fn account_retrieve(
        client: miden_client::Client,
        store: Arc<miden_client::store::web_store::WebStore>,
        keystore: Arc<miden_client::keystore::WebKeyStore<Box<dyn miden_client::crypto::FeltRng>>>,
        model: reactive_stores::Store<payden_model::Model>,
        header: &miden_client::account::AccountHeader,
    ) -> ResultDyn<Self> {
        let account_id = header.id();
        let address = account_id.to_bech32(miden_objects::account::NetworkId::Testnet);
        model.address().set(address);
        Ok(Self { client, account_id, store, keystore, model })
    }

    pub async fn account_generate(
        mut client: miden_client::Client,
        store: Arc<miden_client::store::web_store::WebStore>,
        keystore: Arc<miden_client::keystore::WebKeyStore<Box<dyn miden_client::crypto::FeltRng>>>,
        model: reactive_stores::Store<payden_model::Model>,
    ) -> ResultDyn<Self> {
        let mut account_seed = [0u8; 32];
        client.rng().fill_bytes(&mut account_seed);

        let keypair = miden_client::crypto::SecretKey::with_rng(client.rng());
        let public_key = keypair.public_key();

        let builder = miden_client::account::AccountBuilder::new(account_seed)
            .account_type(miden_client::account::AccountType::RegularAccountUpdatableCode)
            .storage_mode(miden_client::account::AccountStorageMode::Public)
            .with_auth_component(miden_client::account::component::RpoFalcon512::new(public_key))
            .with_component(miden_client::account::component::BasicWallet);

        let (account, seed) = builder.build()?;

        client.add_account(&account, Some(seed), false /* overwrite */).await?;
        keystore.add_key(&miden_client::auth::AuthSecretKey::RpoFalcon512(keypair)).await?;

        // Based off https://github.com/0xMiden/miden-client/blob/eb7643811a724378c63198b80c7d98a564777f0e/crates/web-client/src/account.rs#L48-L77
        let account_storage = account.storage();
        let pub_key_index = u8::from(account.is_faucet());
        let account_pub_key = account_storage.get_item(pub_key_index).unwrap();
        store.fetch_and_cache_account_auth_by_pub_key(account_pub_key.to_hex()).await.unwrap();

        let account_id = account.id();
        let address = account_id.to_bech32(miden_objects::account::NetworkId::Testnet);
        model.address().set(address);

        Ok(Self { client, account_id, store, keystore, model })
    }

    // Based off the [Mint, Consume, and Create Notes] in the Miden Book.
    //
    // [Mint, Consume, and Create Notes]: https://0xmiden.github.io/miden-docs/imported/miden-tutorials/src/rust-client/mint_consume_create_tutorial.html
    pub async fn account_sync(&mut self) -> ResultDyn<()> {
        self.client.sync_state().await?;

        logging::log!("Downloading faucet");

        let (_, faucet) =
            miden_client::account::AccountId::from_bech32("mtst1qppen8yngje35gr223jwe6ptjy7gedn9").unwrap();
        self.client.import_account_by_id(faucet).await.unwrap();

        logging::log!("Downloading faucet - DONE");

        logging::log!("Getting notes");

        let consummable_notes = self.client.get_consumable_notes(Some(self.account_id)).await?;

        if let Some((note, _)) = consummable_notes.first() {
            for fungible in note.assets().iter_fungible() {
                logging::log! {"Note amount: {}", fungible.amount()};
            }
        }

        logging::log!("Getting notes - DONE - {}", consummable_notes.len());

        logging::log!("Building transaction request");
        let note_ids = consummable_notes.iter().map(|(note, _)| note.id()).collect();
        let transaction_request =
            miden_client::transaction::TransactionRequestBuilder::default().build_consume_notes(note_ids)?;
        logging::log!("Building transaction request - DONE");

        logging::log!("Executing transaction");
        let tx_execution_result = self.client.new_transaction(self.account_id, transaction_request).await?;
        logging::log!("Executing transaction - DONE");

        logging::log!("Submitting transaction");
        let prover = Arc::new(miden_client::RemoteTransactionProver::new("https://tx-prover.testnet.miden.io"));
        self.client.submit_transaction_with_prover(tx_execution_result, prover).await?;
        logging::log!("Submitting transaction - DONE");

        logging::log!("Getting Account");
        let account = self.client.get_account(self.account_id).await.unwrap().unwrap();
        logging::log!("Getting Account - DONE");

        logging::log!("Getting balance");
        let vault = account.account().vault();
        let balance_new = vault.get_balance(faucet).unwrap() / 1_000_000;
        logging::log!("Getting balance - DONE");

        logging::log!("New balance is {balance_new}");

        self.model.balance().update(|balance| *balance += *balance + balance_new as f64);

        Ok(())
    }

    pub async fn send(&mut self) {
        todo!()
    }

    pub async fn mint(&mut self) {
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

    async fn store() -> ResultDyn<Arc<miden_client::store::web_store::WebStore>> {
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
