use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde_json::json;
use near_sdk::json_types::Base58PublicKey;
use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault, Promise, PublicKey, Gas,
};

mod nft_approval_receiver;

near_sdk::setup_alloc!();

/// Access key allowance for linkdrop keys.
const ACCESS_KEY_ALLOWANCE: u128 = 1_000_000_000_000_000_000_000_000;

const GAS_FOR_RESOLVE_TRANSFER: Gas = 10_000_000_000_000;
const GAS_FOR_NFT_TRANSFER_CALL: Gas = 25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NFT {
    contract_id: AccountId,
    token_id: TokenId,
    approval_id: u64,
    owner_id: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    accounts: LookupMap<PublicKey, NFT>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            accounts: LookupMap::new(b"a".to_vec()),
        }
    }
    /// Allows given public key to claim sent NFT.
    /// Takes ACCESS_KEY_ALLOWANCE as fee from deposit to cover account creation via an access key.
    #[payable]
    pub fn send(&mut self, public_key: Base58PublicKey, nft: NFT) -> Promise {
        // assert!(
        //     env::attached_deposit() > ACCESS_KEY_ALLOWANCE,
        //     "Attached deposit must be greater than ACCESS_KEY_ALLOWANCE"
        // );
        let pk = public_key.into();
        self.accounts.insert(
            &pk,
            &nft,
        );
        Promise::new(env::current_account_id()).add_access_key(
            pk,
            ACCESS_KEY_ALLOWANCE,
            env::current_account_id(),
            b"claim".to_vec(),
        )
    }

    /// Claim NFT for specific account that are attached to the public key this tx is signed with.
    pub fn claim(&mut self, account_id: AccountId) -> Promise {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Claim only can come from this account"
        );
        assert!(
            env::is_valid_account_id(account_id.as_bytes()),
            "Invalid account id"
        );
        let nft = self
            .accounts
            .remove(&env::signer_account_pk())
            .expect("Unexpected public key");
        Promise::new(env::current_account_id()).delete_key(env::signer_account_pk());
        Promise::new(nft.contract_id)
            .function_call(
                b"nft_transfer".to_vec(),
                json!({
                    "receiver_id": account_id,
                    "token_id": nft.token_id,
                    "approval_id": nft.approval_id,
                    "memo": "",
                })
                .to_string()
                .as_bytes()
                .to_vec(),
                1,
                GAS_FOR_NFT_TRANSFER_CALL,
            )
    }
}
