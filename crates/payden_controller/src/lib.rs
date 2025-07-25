//! Based off the [Miden docs].
//!
//! [Miden docs]: https://0xmiden.github.io/miden-docs/imported/miden-tutorials/src/rust-client/create_deploy_tutorial.html

extern crate alloc;

mod error;
mod faucet;

use leptos::{logging, prelude::*};
use payden_model::*;
use rand::RngCore;

use crate::error::*;

const TESTNET_FAUCET: &str = "mtst1qppen8yngje35gr223jwe6ptjy7gedn9"; // Minting
const TESTNET_PROVER: &str = "https://tx-prover.testnet.miden.io"; // Delegated Proving

#[derive(Clone)]
pub enum ControllerAction {
    Refresh,
    Send { amount: f64, recipient: String },
    Mint { amount: f64 },
}

pub struct Controller {
    client: miden_client::Client,
    account_id: miden_objects::account::AccountId,
    pub model: reactive_stores::Store<payden_model::Model>,
}

// TODO: add back proper error handling
impl Controller {
    pub async fn handle(&mut self, action: ControllerAction) {
        match action {
            ControllerAction::Refresh => {
                if let Err(e) = self.account_sync().await {
                    logging::log!("Failed to refresh account state: {e:?}");
                }
            }
            ControllerAction::Send { amount, recipient } => {
                if let Err(e) = self.send(amount, recipient).await {
                    logging::log!("Failed to send tx: {e:?}");
                }
            }
            ControllerAction::Mint { amount } => {
                if let Err(e) = self.mint(amount).await {
                    logging::log!("Failed to mint assets: {e:?}");
                }
            }
        }
    }

    pub async fn new() -> std::rc::Rc<std::cell::RefCell<Self>> {
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

        let _sync_state = client.sync_state().await.unwrap();

        let model = reactive_stores::Store::new(payden_model::Model::default());

        // Restoring previous account
        let controller = if let Some((header, _)) = client.get_account_headers().await.unwrap().first() {
            logging::log!("Loading previous account");
            Self::account_retrieve(client, store, model, header).await.unwrap()
        } else {
            logging::log!("Creating new account");
            Self::account_generate(client, store, keystore, model).await.unwrap()
        };

        std::rc::Rc::new(controller.into())
    }

    pub async fn account_retrieve(
        client: miden_client::Client,
        store: std::sync::Arc<miden_client::store::web_store::WebStore>,
        model: reactive_stores::Store<payden_model::Model>,
        header: &miden_client::account::AccountHeader,
    ) -> ResultDyn<Self> {
        let account_id = header.id();

        // STEP 1 - Restore account auth cache

        // Based off https://github.com/0xMiden/miden-client/blob/eb7643811a724378c63198b80c7d98a564777f0e/crates/web-client/src/account.rs#L48-L77
        // and https://github.com/0xMiden/miden-client/pull/779
        let account_record = client.get_account(account_id).await?.expect("known address");
        let account = account_record.account();
        let pub_key_index = u8::from(account.is_faucet());
        let account_pub_key = account.storage().get_item(pub_key_index).unwrap();
        store.fetch_and_cache_account_auth_by_pub_key(account_pub_key.to_hex()).await?;

        // Step 2 - Update account address

        let address = account_id.to_bech32(miden_objects::account::NetworkId::Testnet);
        model.address().set(address);

        // Step 3 - Update account balance

        let faucet_id = miden_client::account::AccountId::from_bech32(TESTNET_FAUCET).expect("known address").1;
        let vault = account.vault();
        let balance_new = vault.get_balance(faucet_id).unwrap() / 1_000_000;
        model.balance().set(balance_new as f64);

        Ok(Self { client, account_id, model })
    }

    pub async fn account_generate(
        mut client: miden_client::Client,
        store: std::sync::Arc<miden_client::store::web_store::WebStore>,
        keystore: std::sync::Arc<miden_client::keystore::WebKeyStore<Box<dyn miden_client::crypto::FeltRng>>>,
        model: reactive_stores::Store<payden_model::Model>,
    ) -> ResultDyn<Self> {
        // Step 1 - generate client public/private keys

        let mut account_seed = [0u8; 32];
        client.rng().fill_bytes(&mut account_seed);
        let keypair = miden_client::crypto::SecretKey::with_rng(client.rng());
        let public_key = keypair.public_key();

        // Step 2 - build the account MASM

        let builder = miden_client::account::AccountBuilder::new(account_seed)
            .account_type(miden_client::account::AccountType::RegularAccountUpdatableCode)
            .storage_mode(miden_client::account::AccountStorageMode::Public)
            .with_auth_component(miden_client::account::component::RpoFalcon512::new(public_key))
            .with_component(miden_client::account::component::BasicWallet);

        let (account, seed) = builder.build()?;

        // Step 3 - Update the client

        client.add_account(&account, Some(seed), false /* overwrite */).await?;
        keystore.add_key(&miden_client::auth::AuthSecretKey::RpoFalcon512(keypair)).await?;

        // Based off https://github.com/0xMiden/miden-client/blob/eb7643811a724378c63198b80c7d98a564777f0e/crates/web-client/src/account.rs#L48-L77
        // and https://github.com/0xMiden/miden-client/pull/779
        let account_storage = account.storage();
        let pub_key_index = u8::from(account.is_faucet());
        let account_pub_key = account_storage.get_item(pub_key_index).unwrap();
        store.fetch_and_cache_account_auth_by_pub_key(account_pub_key.to_hex()).await.unwrap();

        // Step 4 - Update account address

        let account_id = account.id();
        let address = account_id.to_bech32(miden_objects::account::NetworkId::Testnet);
        model.address().set(address);

        Ok(Self { client, account_id, model })
    }

    // Based off the [Mint, Consume, and Create Notes] in the Miden Book.
    //
    // [Mint, Consume, and Create Notes]: https://0xmiden.github.io/miden-docs/imported/miden-tutorials/src/rust-client/mint_consume_create_tutorial.html
    pub async fn account_sync(&mut self) -> ResultDyn<()> {
        self.client.sync_state().await?;

        logging::log!("Getting notes");
        let consummable_notes = self.client.get_consumable_notes(Some(self.account_id)).await?;

        logging::log!("Building transaction request");
        let note_ids = consummable_notes.iter().map(|(note, _)| note.id()).collect();
        let transaction_request =
            miden_client::transaction::TransactionRequestBuilder::new().build_consume_notes(note_ids)?;

        logging::log!("Executing transaction");
        let tx_execution_result = self.client.new_transaction(self.account_id, transaction_request).await?;

        logging::log!("Submitting transaction");
        let prover = std::sync::Arc::new(miden_client::RemoteTransactionProver::new(TESTNET_PROVER));
        self.client.submit_transaction_with_prover(tx_execution_result, prover).await?;

        let faucet_id = miden_client::account::AccountId::from_bech32(TESTNET_FAUCET).expect("known address").1;
        let account = self.client.get_account(self.account_id).await?.expect("known address");
        let vault = account.account().vault();
        let balance_new = vault.get_balance(faucet_id).expect("balance update as part of tx") / 1_000_000;

        self.model.balance().set(balance_new as f64);

        Ok(())
    }

    pub async fn send(&mut self, amount: f64, recipient: String) -> ResultDyn<()> {
        self.client.sync_state().await?;

        logging::log!("Creating assets");
        let send_amount = amount as u64 * 1_000_000;
        let (_, recipient) = miden_client::account::AccountId::from_bech32(&recipient)?;
        let faucet_id = miden_client::account::AccountId::from_bech32(TESTNET_FAUCET).expect("known address").1;

        let fungible_asset = miden_client::asset::FungibleAsset::new(faucet_id, send_amount)?;
        let payment_data = miden_client::transaction::PaymentNoteDescription::new(
            vec![fungible_asset.into()],
            self.account_id,
            recipient,
        );

        logging::log!("Building transaction request");
        let transaction_request = miden_client::transaction::TransactionRequestBuilder::new().build_pay_to_id(
            payment_data,
            miden_client::note::NoteType::Public,
            self.client.rng(),
        )?;

        logging::log!("Executing transaction");
        let tx_execution_result = self.client.new_transaction(self.account_id, transaction_request).await?;

        logging::log!("Submitting transaction");
        let prover = std::sync::Arc::new(miden_client::RemoteTransactionProver::new(TESTNET_PROVER));
        self.client.submit_transaction_with_prover(tx_execution_result, prover).await?;

        self.model.balance().update(|balance| *balance -= amount);

        Ok(())
    }

    pub async fn mint(&mut self, amount: f64) -> ResultDyn<()> {
        logging::log!("Retrieving faucet challenge");
        let faucet = faucet::FaucetClient::new("https://faucet.testnet.miden.io".to_string());
        let challenge = faucet.get_challenge(&self.account_id).await?;

        logging::log!("Solving challenge");
        let nonce = faucet::solve_challenge(&challenge);

        logging::log!("Requesting tokens");
        faucet.request_tokens(&self.account_id, false, amount as u64, &challenge, nonce).await?;

        Ok(())
    }

    fn rpc_api() -> std::sync::Arc<impl miden_client::rpc::NodeRpcClient + Send> {
        let endpoint = miden_client::rpc::Endpoint::testnet();
        let timeout_ms = 10_000;
        std::sync::Arc::new(miden_client::rpc::TonicRpcClient::new(&endpoint, timeout_ms))
    }

    fn rng() -> ResultDyn<Box<dyn miden_client::crypto::FeltRng>> {
        let mut seed = [0u64; 4];
        rand::fill(&mut seed);
        Ok(Box::new(miden_client::crypto::RpoRandomCoin::new(seed.map(miden_client::Felt::new))))
    }

    async fn store() -> ResultDyn<std::sync::Arc<miden_client::store::web_store::WebStore>> {
        Ok(std::sync::Arc::new(miden_client::store::web_store::WebStore::new().await.map_err(|_| WebStoreInitError)?))
    }

    fn keystore()
    -> ResultDyn<std::sync::Arc<miden_client::keystore::WebKeyStore<Box<dyn miden_client::crypto::FeltRng>>>> {
        let rng = Self::rng()?;
        Ok(std::sync::Arc::new(miden_client::keystore::WebKeyStore::new(rng)))
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
