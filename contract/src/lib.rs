/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, AccountId, Balance, near_bindgen, log};


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    users: LookupMap<AccountId, User>,

}

//User will have 2 properties, A mapping to see how much money they own, and an integer to see the last interaction with our contract.
struct User {
    period_balance: LookupMap<u16,Balance>,
    latest_period: u16
}


// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            users: LookupMap::new(b"m")
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }

    //baturay algorithm, we allow 1 for loop
    fn optimize_user_balances(&mut self, AccountId user_account) {

    }

    //deposit function for launchpad
    //todo: if user has account on contract, add balance to account, else create a new account and add balance to it.
    #[payable]
    pub fn deposit_balance(&mut self) {
        let sender = env::predecessor_account_id();
        let deposit_amount = env::
        self.users.insert
    }

    pub fn get_user_balance(&mut self, AccountId user_account) {
        match self.users.get(&user_account) {
            Some(value) => {
                let log_message = format!("Value from LookupMap is {:?}", value.clone());
                env::log(log_message.as_bytes());
                value
            },
            None => "not found".to_string()
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
