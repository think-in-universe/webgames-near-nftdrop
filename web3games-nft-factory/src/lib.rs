use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde_json::json;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, near_bindgen, PanicOnDefault, Promise, Balance, Gas,
};

near_sdk::setup_alloc!();

const NO_DEPOSIT: Balance = 0;

/// Initial balance for the NFT contract to cover storage and related.
const NFT_INIT_BALANCE: Balance = 5_000_000_000_000_000_000_000_000; // 5e24yN, 5N

/// Gas to initialize NFT contract.
const NFT_NEW: Gas = 1_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn deploy_nft(&mut self, contract_name: String, owner_id: ValidAccountId, metadata: NFTContractMetadata) {
        assert!(
            env::attached_deposit()
                >= NFT_INIT_BALANCE,
            "Not enough attached deposit to complete nft creation"
        );

        let nft_account_id = format!("{}.{}", contract_name, env::current_account_id());

        Promise::new(nft_account_id)
            .create_account()
            .transfer(NFT_INIT_BALANCE)
            .add_full_access_key(env::signer_account_pk())
            .deploy_contract(
                include_bytes!("../res/web3games_nft.wasm").to_vec(),
            )
            .function_call(
                b"new".to_vec(),
                json!({
                "owner_id": owner_id,
                "metadata": metadata,
            })
                    .to_string()
                    .into_bytes(),
                NO_DEPOSIT,
                NFT_NEW,
            );
    }
}

