use crate::*;
use near_sdk::PromiseOrValue;

use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApprovalReceiver;

/// approval callbacks from NFT Contracts
#[near_bindgen]
impl NonFungibleTokenApprovalReceiver for Contract {
	#[payable]
	fn nft_on_approve(
		&mut self,
		token_id: TokenId,
		owner_id: AccountId,
		approval_id: u64,
		msg: String,
	) -> PromiseOrValue<String> {
		let contract_id: AccountId = env::predecessor_account_id();
		let public_key: Base58PublicKey = near_sdk::serde_json::from_str(&msg).expect("Valid send args");
		let nft = NFT {
			contract_id,
			owner_id,
			token_id,
			approval_id,
		};
		self.send_nft(
			public_key,
			nft,
		);

		PromiseOrValue::Value("OK".to_string())
	}
}
