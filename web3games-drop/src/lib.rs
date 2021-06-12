#![feature(gen_future)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58PublicKey};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde_json::{json};
use near_sdk::{env, near_bindgen, AccountId, Promise, PublicKey, wee_alloc,PanicOnDefault};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/// Access key allowance for NFTDrop keys.
const ACCESS_KEY_ALLOWANCE: u128 = 1_000_000_000_000_000_000_000_000;


pub const GAS: u64 = 20_000_000_000_000;

pub type NftId = u64;

pub trait Drop{
    fn add(&mut self, public_key: Base58PublicKey, token_id: NftId) -> Promise;
    fn claim(&mut self, account_id: AccountId) -> u64;
}


#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct NFTDrop {
    pub accounts: UnorderedMap<PublicKey, NftId>,
}

#[near_bindgen]
impl NFTDrop{
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid.");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            accounts:UnorderedMap::new(b"token-belongs".to_vec()),
        }
    }
}

#[near_bindgen]
impl Drop for NFTDrop {
    /// Add NftId to account , mapping  Account => TokenID
    #[payable]
    fn add(&mut self, public_key: Base58PublicKey, token_id: NftId) -> Promise {
        let pk = public_key.into();
        self.accounts.insert(
            &pk,
            &token_id,
        );
        Promise::new(env::current_account_id()).add_access_key(
            pk,
            ACCESS_KEY_ALLOWANCE,
            env::current_account_id(),
            b"claim".to_vec(),
        )
    }

    /// Claim tokens for specific account that are attached to the public key this tx is signed with.
    fn claim(&mut self, account_id: AccountId) -> u64 {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Claim only can come from this account"
        );
        assert!(
            env::is_valid_account_id(account_id.as_bytes()),
            "Invalid account id"
        );
        let nft_id = self.accounts.get(&env::signer_account_pk());
        let _amount = self
            .accounts
            .remove(&env::signer_account_pk())
            .expect("Unexpected public key");
        Promise::new(env::current_account_id()).delete_key(env::signer_account_pk());
        env::promise_create(
            "zombie3.testnet".to_string(),
            b"transfer_from",
            json!({ "from": env::current_account_id(), "to": account_id, "token_id": nft_id }).to_string().as_bytes(),
            0,
            GAS,
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};
    use std::convert::TryInto;

    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_add() {
        let context = get_context("zombie.testnet".to_string(),1024*300);
        testing_env!(context);
        let mut contract = NFTDrop::new("zombie.testnet".to_string());
        let pk: Base58PublicKey = "qSq3LoufLvTCTNGC3LJePMDGrok8dHMQ5A1YD9psbiz"
            .try_into()
            .unwrap();
        contract.add(pk,1);
    }
}