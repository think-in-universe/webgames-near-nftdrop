#![feature(gen_future)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,near_bindgen, AccountId,PanicOnDefault};
use near_sdk::collections::UnorderedMap;
use near_sdk::test_utils::accounts;
use near_sdk::borsh::maybestd::borrow::Borrow;



#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

pub trait ERC721 {
    // view function,return the owner address
    fn owner_of(&self, token_id: TokenId) -> AccountId;
    // view function,return the Approved Account
    fn get_approved(&self, token_id:TokenId) -> AccountId;
    // view function,return the Approved all bool
    fn is_approved_for_all(&self, owner:AccountId, operator:AccountId) -> bool;
    // view function,check Authority
    fn is_approved_or_owner(&self, spender:AccountId, token_id:TokenId) -> bool;
    // view function,return the balance number
    fn balance_of(&self, owner: AccountId) -> u128;
    // view function,check nft
    fn exists(&self,token_id:TokenId) -> bool;
    // view function,return the token URI information
    fn get_uri(&self, token_id:TokenId) -> String;


    // change function,approve
    fn approve(&mut self,to:AccountId,token_id:TokenId);
    // change function,setApproval
    fn set_approval_for_all(&mut self, operator:AccountId, approved:bool);
    // change function,safeMint
    fn mint(&mut self,to:AccountId,token_id:TokenId);
    // change function,safeBurn
    fn burn(&mut self,token_id:TokenId);
    // change function,transfer_from
    fn transfer_from(&mut self, from:AccountId, to:AccountId, token_id:TokenId);
    // change function,set uri
    fn token_uri(&mut self, token_id: TokenId, uri:String);
}

pub type TokenId = u64;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize,PanicOnDefault)]
pub struct Near721 {
    owners:UnorderedMap<TokenId,AccountId>,
    balances:UnorderedMap<AccountId,u128>,
    token_approvals:UnorderedMap<TokenId,AccountId>,
    operator_approvals:UnorderedMap<(AccountId, AccountId),bool>,
    uri:UnorderedMap<TokenId,String>
}

#[near_bindgen]
impl Near721 {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid.");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owners:UnorderedMap::new(b"token-belongs".to_vec()),
            balances:UnorderedMap::new(b"token-balance".to_vec()),
            token_approvals:UnorderedMap::new(b"token-approvals".to_vec()),
            operator_approvals:UnorderedMap::new(b"token-Authority".to_vec()),
            uri:UnorderedMap::new(b"token-URI".to_vec()),
        }
    }
}

#[near_bindgen]
impl ERC721 for Near721 {
    /// get tokenID owner account
    fn owner_of(&self, token_id: TokenId) -> AccountId {
         match self.owners.get(&token_id) {
            Some(account) => account,
            None => env::panic(b"No owner of the token ID specified")
        }.to_string()
    }

    /// get approved account
    fn get_approved(&self, token_id:TokenId) -> AccountId {
        // have FNT or not
        let exists_result = self.exists(token_id);

        if !exists_result  {
            // no nft
            env::panic(b"approved query for nonexistent token")
        }
        // get tokenID approval
        let approved = self.token_approvals.get(&token_id);
        // if have then output account or ""
        return match approved {
            Some(account) => account,
            None => "".to_string()
        }.to_string()
    }

    /// get ApprovedForAll
    fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool{
        // get Authority
        let all = self.operator_approvals.get(&(owner, operator));
        return match all {
            None => false,
            Some(_result) => true
        };
    }

    /// check Authority
    fn is_approved_or_owner(&self, spender:AccountId, token_id:TokenId) -> bool {
        //have FNT or not
        let is_exists = self.exists(token_id);

        if !is_exists {
            // not nft
            env::panic(b"operator query for nonexistent token")
        };
        // have nft and check
        //input account = get tokenID owner account / account have approved / get ApprovedForAll
        let result =  spender == self.owner_of(token_id) || self.get_approved(token_id) == spender || self.is_approved_for_all(self.owner_of(token_id), spender);
        result
    }

    /// get number of TokenId owner <1>
    fn balance_of(&self, owner: AccountId) -> u128 {
        match self.balances.get(&owner) {
            Some(number) => number,
            None => 0
       }
    }

    /// have FNT or not
    fn exists(&self, token_id: TokenId) -> bool {
        // ture =  have NFT
        // false = no NFT and address = alice
        let owner = self.owners.get(&token_id);
        let result = match owner {
            Some(account) => account,
            None => "".to_string()
        };
        return if result != "alice.testnet".to_string() && result != "".to_string() {
            true
        } else { false };
    }

    /// get token_uri of TokenID
    fn get_uri(&self, token_id:TokenId) -> String {
        match self.uri.get(&token_id) {
            Some(uri) => uri,
            None => env::panic(b"no URI")
        }
    }

    /// give tokenID approve to input account
    fn approve(&mut self, to: AccountId, token_id: TokenId) {
        //get tokenID owner account
        let owner = self.owner_of(token_id);

        if to == owner{
            // same account
            env::panic(b"approval to current owner.")
        };
        // get tx account
        let signer = env::predecessor_account_id();
        // get Authority
        let check_result = self.is_approved_for_all(String::from(&owner), String::from(&signer));
        // check Authority
        if (&owner == &signer || check_result) == false {
            env::panic(b"NEAR721: approve caller is not owner nor approved for all")
        }
        self.token_approvals.insert(&token_id, &to);
    }

    /// input != tx account and give operator_approvals
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) {
        // get tx account
        let signer:AccountId = env::predecessor_account_id();

        if operator == signer{
            // input = tx account
            env::panic(b"approve to caller")
        }
        // input != tx account and give operator_approvals
        self.operator_approvals.insert(&(signer, operator), &approved);
    }

    /// mint nft
    fn mint(&mut self,to: AccountId,token_id:TokenId) {
        if to == "alice.testnet".to_string(){
            env::panic(b"mint to the zero address")
        };
        if self.exists(token_id){
            env::panic(b"token already minted")
        };
        let number = match self.balances.get(&env::predecessor_account_id()){
            Some(number) => number,
            None => 0
        };
        self.balances.insert(&to, &(number + 1));
        self.owners.insert(&token_id,&to);
    }

    ///burn nft
    fn burn(&mut self, token_id: u64) {
        // get tokenID -> owner account
        let token_owner_account_id = self.owner_of(token_id);

        // Clear approvals
        self.approve("".to_string(), token_id);
        // balance - 1
        let number = match self.balances.get(&env::predecessor_account_id()){
            Some(number) => number,
            None => 0
        };
        self.balances.insert(&token_owner_account_id, &(number - 1));
        // remove index
        self.owners.remove(&token_id);
    }

    /// transfer from - > to address
    fn transfer_from(&mut self, from:AccountId, to:AccountId, token_id:TokenId){
        if self.is_approved_or_owner(env::predecessor_account_id(), token_id) == false {
            // no Authority
            env::panic(b"transfer caller is not owner nor approved")
        };

        // false = send it to zero address
        if to == "".to_string(){
            env::panic(b"transfer to the zero address")
        };

        // clear last approve account
        self.approve("".to_string(),token_id);

        //from account balance number -1 ,to + 1 and change the owners info
        let from_number = match self.balances.get(&from){
            Some(number) => number,
            None => 0
        };
        let to_number = match self.balances.get(&from){
            Some(number) => number,
            None => 0
        };
        self.balances.insert(&from,&(from_number - 1));
        self.balances.insert(&to,&(to_number + 1));
        self.owners.insert(&token_id,&to);
    }

    /// set NFT URI info
    fn token_uri(&mut self, token_id: TokenId, uri:String)  {
        // if alice address = false
        if !self.exists(token_id)  {
            // no tokenID
            env::panic(b"URI query for nonexistent token")
        }
        // get owner
        let owner = self.owners.get(&token_id);
        // check owner have
        let owner_have = match owner {
            Some(owner) => owner,
            None => "".to_string()
        };
        // check ownership
        let signer = env::predecessor_account_id();
        if signer != owner_have{
            env::panic(b"no ownership,can not set URI")
        }
        self.uri.insert(&token_id, &uri);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};

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
    #[should_panic(expected = r#"No owner of the token ID specified"#)]
    fn check_owner_of_none()  {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let contract = Near721::new("zombie.testnet".to_string());
        assert_eq!(contract.owner_of(1),"".to_string())
    }

    #[test]
    #[should_panic(expected = r#"approved query for nonexistent token"#)]
    fn get_approved_check()  {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        assert_eq!(contract.get_approved(1),"zombie.testnet".to_string());
    }

    #[test]
    fn get_approved_check_have()  {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie.testnet".to_string(),1);
        assert_eq!(contract.get_approved(1),"".to_string());
    }

    #[test]
    fn is_approved_for_all_check_true()  {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.set_approval_for_all("zombie2.testnet".to_string(),true);
        assert_eq!(contract.is_approved_for_all("zombie.testnet".to_string(),"zombie2.testnet".to_string()),true)
    }

    #[test]
    fn is_approved_for_all_check_false()  {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.set_approval_for_all("zombie2.testnet".to_string(),true);
        assert_eq!(contract.is_approved_for_all("zombie.testnet".to_string(),"zombie3.testnet".to_string()),false)
    }

    #[test]
    #[should_panic(expected = r#"operator query for nonexistent token"#)]
    fn is_approved_or_owner_check()  {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.is_approved_or_owner("zombie.testnet".to_string(),1);
    }

    #[test]
    fn is_approved_or_owner_check_owner_of_true()  {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie.testnet".to_string(),1);
        assert_eq!(contract.is_approved_or_owner("zombie.testnet".to_string(),1),true)
    }

    #[test]
    fn is_approved_or_owner_check_approve_true()  {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie.testnet".to_string(),1);
        contract.approve("zombie1.testnet".to_string(),1);
        assert_eq!(contract.is_approved_or_owner("zombie1.testnet".to_string(),1),true)
    }

    #[test]
    fn is_approved_or_owner_check_is_approved_for_all_true()  {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie.testnet".to_string(),1);
        contract.set_approval_for_all("zombie1.testnet".to_string(),true);
        assert_eq!(contract.is_approved_for_all("zombie.testnet".to_string(), "zombie1.testnet".to_string()),true);
        assert_eq!(contract.is_approved_or_owner("zombie1.testnet".to_string(),1),true)
    }

    #[test]
    fn exists_check(){
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie.testnet".to_string(),1);
        assert_eq!(contract.exists(0),false);
        assert_eq!(contract.exists(1),true)
    }

    #[test]
    #[should_panic(expected = r#"no URI"#)]
    fn get_uri_check() {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie.testnet".to_string(),0);
        contract.mint("zombie.testnet".to_string(),1);
        contract.token_uri(1,"http".to_string());
        assert_eq!(contract.get_uri(1),"http".to_string());
        assert_eq!(contract.get_uri(0),"".to_string());
    }

    #[test]
    #[should_panic(expected = r#"No owner of the token ID specified"#)]
    fn approve_check_notTokenId() {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.approve("zombie.testnet".to_string(),1);
    }

    #[test]
    #[should_panic(expected = r#"approval to current owner"#)]
    fn approve_check_sameAccount() {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie.testnet".to_string(),1);
        contract.approve("zombie.testnet".to_string(),1);
    }

    #[test]
    #[should_panic(expected = r#"approve caller is not owner nor approved for all"#)]
    fn approve_check_authority() {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie1.testnet".to_string(),1);
        contract.approve("zombie.testnet".to_string(),1);
    }

    #[test]
    fn approve_check_authority_ok() {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie.testnet".to_string(),1);
        contract.set_approval_for_all("zombie1.testnet".to_string(), true);
        contract.approve("zombie1.testnet1".to_string(),1)
    }

    #[test]
    #[should_panic(expected = r#"approve to caller"#)]
    fn set_approval_for_all_check_same_account() {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.set_approval_for_all("zombie.testnet".to_string(),true);
    }

    #[test]
    #[should_panic(expected = r#"mint to the zero address"#)]
    fn check_mint_zero() {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("alice.testnet".to_string(),0);
    }

    #[test]
    #[should_panic(expected = r#"token already minted"#)]
    fn check_mint_have(){
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie.testnet".to_string(),0);
        contract.mint("zombie.testnet".to_string(),0);
    }

    #[test]
    fn burn_check_haveNFT(){
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie.testnet".to_string(),1);
        contract.burn(1)
    }
    #[test]
    #[should_panic(expected = r#"transfer caller is not owner nor approved"#)]
    fn transfer_from_not_owner_check(){
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("alice1.testnet".to_string(),0);
        contract.transfer_from("alice3.testnet".to_string(),"alice2.testnet".to_string(),0);
    }

    #[test]
    #[should_panic(expected = r#"URI query for nonexistent token"#)]
    fn token_uri_check_exists(){
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.token_uri(1,"".to_string());
    }

    #[test]
    #[should_panic(expected = r#"no ownership,can not set URI"#)]
    fn token_uri_check_owner(){
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Near721::new("zombie.testnet".to_string());
        contract.mint("zombie1.testnet".to_string(),0);
        contract.token_uri(0,"".to_string());
    }
}