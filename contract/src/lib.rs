/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, AccountId, Balance, near_bindgen};


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    users: LookupMap<AccountId, User>,
    start_period: u64
}

//User will have 2 properties, A mapping to see how much money they own, and an integer to see the last interaction with our contract.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct User {
    period_balance: LookupMap<u64,Balance>,
    latest_period: u64
}


// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            users: LookupMap::new(b"m"),
            start_period: env::block_timestamp()
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    //baturay algorithm, we allow 1 for loop
    fn optimize_user_balances(&mut self,  user_account: AccountId) {

    }

    //deposit function for launchpad
    //todo: if user has account on contract, add balance to account, else create a new account and add balance to it.
    #[payable]
    pub fn deposit_balance(&mut self) {
        let sender: AccountId = env::signer_account_id();
        let deposit_amount: Balance = env::attached_deposit();
        let current: u64 = (env::block_timestamp() - self.start_period.clone()) / 37;
        let user_balance = self.get_user_balance( &sender).clone();
        let mut map_struct = LookupMap::new(b"m");
        let final_balance = deposit_amount + user_balance;
        map_struct.insert(&current,&final_balance);
        self.users.insert(&sender,&User {period_balance: map_struct,latest_period: current});
    }
    pub fn get_user_balance(&mut self,  user_account: &AccountId) -> u128{
        match self.users.get(&user_account) {
            Some(value)  => {
                let log_message = format!("Value from LookupMap is {:?}", value.period_balance.get(&0).clone());
                match value.period_balance.get(&0) {
                    Some(val) =>{
                        val
                    }
                    None=> 0
                }
            },
            None => 0
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
