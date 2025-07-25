//! Based off the official [Miden Faucet].
//!
//! API endpoints used here can be found under the [faucet router implementation].
//!
//! [Miden Faucet]: https://github.com/0xMiden/miden-faucet/tree/next
//! [faucet router implementation]: https://github.com/0xMiden/miden-faucet/blob/next/bin/faucet/src/server/mod.rs

use miden_client::{
    account::AccountId,
    utils::{Deserializable, Serializable},
};
use rand::Rng;
use serde::Deserialize;
use sha3::{Digest, Sha3_256};

use crate::ResultDyn;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiKey([u8; 32]);

// Challenge struct matching the faucet implementation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Challenge {
    /// The target used to validate the challenge solution. A lower target makes the challenge more
    /// difficult to solve. A solution is valid if the hash `H(challenge, nonce)`, interpreted as a
    /// big-endian u64 from the first 8 bytes, is less than this target value.
    pub target: u64,
    /// The timestamp of the challenge creation.
    pub timestamp: u64,
    /// The account that requested the challenge.
    pub account_id: AccountId,
    /// The API key used to request the challenge.
    pub api_key: ApiKey,
    /// Deterministically generated signature of the challenge.
    pub signature: [u8; 32],
}

impl Challenge {
    const CHALLENGE_ENCODED_SIZE: usize = 95;

    /// Encodes the challenge into a hex string.
    pub fn encode(&self) -> String {
        let mut bytes = Vec::with_capacity(Self::CHALLENGE_ENCODED_SIZE);
        bytes.extend_from_slice(&self.target.to_le_bytes());
        bytes.extend_from_slice(&self.timestamp.to_le_bytes());
        bytes.extend_from_slice(&self.account_id.to_bytes());
        bytes.extend_from_slice(&self.api_key.0);
        bytes.extend_from_slice(&self.signature);
        format!("0x{}", hex::encode(bytes))
    }

    /// Decodes a challenge from a hex string.
    pub fn decode(hex_str: &str) -> Result<Self, String> {
        let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        let bytes = hex::decode(hex_str).map_err(|e| format!("Failed to decode hex: {}", e))?;

        if bytes.len() != Self::CHALLENGE_ENCODED_SIZE {
            return Err(format!(
                "Invalid challenge length: expected {}, got {}",
                Self::CHALLENGE_ENCODED_SIZE,
                bytes.len()
            ));
        }

        let target = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let timestamp = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        let account_id = AccountId::read_from_bytes(&bytes[16..31]).unwrap();
        let mut api_key_bytes = [0u8; 32];
        api_key_bytes.copy_from_slice(&bytes[31..63]);
        let api_key = ApiKey(api_key_bytes);

        let mut signature = [0u8; 32];
        signature.copy_from_slice(&bytes[63..95]);

        Ok(Challenge { target, timestamp, account_id, api_key, signature })
    }

    /// Validates a proof-of-work solution.
    pub fn validate_pow(&self, nonce: u64) -> bool {
        let mut hasher = Sha3_256::new();
        hasher.update(self.encode());
        hasher.update(nonce.to_be_bytes());
        let hash = hasher.finalize();

        let number = u64::from_be_bytes(hash[..8].try_into().unwrap());
        number < self.target
    }
}

// API response structure
#[derive(Debug, Deserialize)]
struct ChallengeResponse {
    challenge: String,
    target: u64,
    timestamp: u64,
}

// HTTP client for interacting with the faucet
pub struct FaucetClient {
    base_url: String,
    client: reqwest::Client,
}

impl FaucetClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url, client: reqwest::Client::new() }
    }

    /// Gets a PoW challenge from the faucet.  This is used in combination with a rate limit to
    /// make sure the faucet is not flooded with requests.
    ///
    /// This does mean that we risk being rate limited if passing by the official faucet, might be
    /// worth it to deploy our own version.
    pub async fn get_challenge(&self, account_id: &AccountId) -> ResultDyn<Challenge> {
        let url = format!("{}/pow", self.base_url);
        let response = self.client.get(&url).query(&[("account_id", &account_id.to_hex())]).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Faucet API error: {}", error_text).into());
        }

        let challenge_response: ChallengeResponse = response.json().await?;

        let challenge = Challenge::decode(&challenge_response.challenge)?;

        if challenge.target != challenge_response.target {
            return Err("Challenge target mismatch".into());
        }
        if challenge.timestamp != challenge_response.timestamp {
            return Err("Challenge timestamp mismatch".into());
        }

        Ok(challenge)
    }

    pub async fn request_tokens(
        &self,
        account_id: &AccountId,
        is_private_note: bool,
        asset_amount: u64,
        challenge: &Challenge,
        nonce: u64,
    ) -> ResultDyn<()> {
        let url = format!("{}/get_tokens", self.base_url,);

        self.client
            .get(&url)
            .query(&[
                ("account_id", account_id.to_hex()),
                ("is_private_note", is_private_note.to_string()),
                ("asset_amount", asset_amount.to_string()),
                ("challenge", challenge.encode()),
                ("nonce", nonce.to_string()),
            ])
            .send()
            .await?;

        Ok(())
    }
}

/// Find a solution to the PoW challenge posed by the faucet
///
/// As per the [miden faucet].
///
/// [miden faucet]: https://github.com/0xMiden/miden-faucet/blob/46022000c193f1571e43ff857228bff3cff8504d/bin/faucet/src/server/resources/index.js#L217
pub fn solve_challenge(challenge: &Challenge) -> u64 {
    let mut rng = rand::rng();
    loop {
        let nonce = rng.random();
        if challenge.validate_pow(nonce) {
            return nonce;
        }
    }
}
