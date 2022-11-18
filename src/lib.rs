/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};use near_contract_standards::fungible_token::core::FungibleTokenCore;
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{ env, log,PromiseResult, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue,Promise, serde_json::json,Gas};
use near_sdk::require;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;

const XCC_GAS: Gas = Gas(10u64.pow(14));
const NO_DEPOSIT: Balance = 0;
const NO_ARGS: Vec<u8> = vec![];
/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */


//STRUCTS AND CONSTANTS

const WEEKANDMONTH:u128 = 3196800000000000;
const MONTH:u128 = 2592000000000000;
// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    users: LookupMap<AccountId, Balance>,
    total: Balance,
    tokens: LookupMap<AccountId,Balance>,
    stake_called: u128
}

//User will have 2 properties, A mapping to see how much money they own, and an integer to see the last interaction with our contract.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct User {
    period_balance: LookupMap<u64,Balance>,
    latest_period: u64,
}



// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            users: LookupMap::new(b"m"),
            total: 0,
            tokens: LookupMap::new(b"m"),
            stake_called: 0,
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {

    //NOTICE: edited for NEAR hackathon, the implementation of the algorithms will be done later. 
    //INITIALIZER
    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn init() -> Self {
      Self {
        users: LookupMap::new(b"m"),
        total: 0,
        tokens: LookupMap::new(b"m"),
        stake_called: 0
      }
      
    }
    #[payable]
    pub fn deposit_balance(&mut self) {
        require!(u128::from(env::block_timestamp()) >= self.stake_called + MONTH, "staking period is not over yet");
        //predecessor account id for now, check how this works with proxy's, may change to signer account id later.
        let sender: AccountId = env::predecessor_account_id();
        let deposit_amount: Balance = env::attached_deposit();
        let user_balance = self.get_user_balance( &sender).clone();
        let final_balance = deposit_amount + user_balance;
        self.total += deposit_amount;
        self.users.insert(&sender,&final_balance);
    }
    #[payable]
    pub fn withdraw_balance(&mut self, amount: Balance){
        require!(u128::from(env::block_timestamp()) >= self.stake_called + MONTH, "staking period is not over yet");
        let sender: AccountId = env::predecessor_account_id();
        let user_balance = self.get_user_balance( &sender).clone();
        //user can only withdraw money not in current period current = currentblock- initblock / 37 days
        require!(user_balance >=amount, "not enough balance");//block useless requests to contract.
        let final_balance = user_balance - amount ;
        self.users.insert(&sender,&final_balance);
        self.total -= amount;
        Promise::new(sender).transfer(amount);
        
    }
    //QUICK WORKING PROJECT BY EREN

        //ADMIN 
        pub fn stake(&mut self, pool: AccountId){
            require!(u128::from(env::block_timestamp()) >= self.stake_called + WEEKANDMONTH, "staking period or withdrawing period is not over yet");
            
            
            Promise::new(pool)
        .function_call("deposit_and_stake".to_owned(), "{}".into(), self.total , near_sdk::Gas(45000000000000));
        
            self.stake_called = u128::from(env::block_timestamp().clone());
        

        }
        pub fn unstake(&mut self, pool: AccountId){
            require!(u128::from(env::block_timestamp()) >= self.stake_called + MONTH, "staking period is not over yet");
            Promise::new(pool.clone())
            .function_call("unstake_all".to_owned(), "{}".into(), 0 , near_sdk::Gas(45000000000000) );
        
            Promise::new(pool.clone())
            .function_call("withdraw_all".to_owned(), "{}".into(), 0 , near_sdk::Gas(45000000000000) );

        }
        //ONLY ME
        #[payable]
        pub fn start_launch(&mut self, account_of_token: AccountId, amount_to_sell: Balance){
            require!(<AccountId as Into<String>>::into(env::predecessor_account_id()) == "erentester.testnet", "Owner's method");
            self.tokens.insert(&account_of_token,&amount_to_sell);
        }
    

    #[payable]
    pub fn purchase(&mut self,  token_account: AccountId){
        //calculate user purchase power
        //need a gate here to stop multiple pruchases
        let purchasing_power = self.get_user_balance(&env::predecessor_account_id()).clone() * self.get_tokens(&token_account).clone()/ self.get_total_balance().clone();
        require!(env::attached_deposit() >= purchasing_power, "need to attach deposit equal to your purchasing power");
        //create tx
        match Promise::new(token_account.clone())
        .function_call("ft_transfer".to_owned(),format!("{{\"receiver_id\": \"{}\", \"amount\": \"{}\"}}",env::predecessor_account_id() ,purchasing_power).into(), 1, XCC_GAS )
        {Failed => {
            log!(format!("Promise failed."));
            
        }
        NotReady => {
            log!(format!("Promise is not ready yet."));
            
        }
        Successful => {

        }}
    }
    //OPTIMIZERS
    //talk to baturay and eren for more information about this algorithm.
    /*
    #[private]
    fn optimize_user_balances(&mut self,  user_account: &AccountId, period: u64) {
        let mut iter = self.get_latest_period(&user_account);
        if  iter != period {
            let mut bal:u128 = 0;
            while iter < period {
                bal += self.get_user_balance(&user_account,&iter) ;
                iter+= 1;
            }
            let mut map_struct = LookupMap::new(b"m");
            //iter and period same value now
            map_struct.insert(&(period),&bal);
            
            let mut map_struct_purchase_period = LookupMap::new(b"m");
            map_struct_purchase_period.insert(&period,&self.get_user_purchases_in_given_period(&user_account,&period));
            self.users.insert(&user_account,&User {period_balance: map_struct,latest_period: period, purchases_in_period: map_struct_purchase_period});
        }
    
    }
    

    #[private]
    fn optimize_total_balance(&mut self,period: u64) {
        let mut iter = self.latest_optimization.clone();
        if  iter != period {
            let mut bal:u128 = 0;
            while iter < period {
                bal += self.get_total_balance(&iter);
                iter+= 1;
            }
            //iter and period same value now
            self.total.insert(&(period),&bal);
            
        }
    
    }

   
    
    //USER FUNCTIONS

    //deposit for each period
    #[payable]
    pub fn deposit_balance(&mut self) {
        let sender: AccountId = env::signer_account_id();
        let deposit_amount: Balance = env::attached_deposit();
        
        //timestamp is second * 1.000.000.000
        //3_196_800_000_000_000 = 37 days in nanosec.
        let current: u64 = (env::block_timestamp() - self.initial_stake_called_at.clone()) / WEEKANDMONTH ;

        //add user balance
        let user_balance = self.get_user_balance( &sender, &current).clone();
        let mut map_struct = LookupMap::new(b"m");
        let final_balance = deposit_amount + user_balance;
        map_struct.insert(&current,&final_balance);

        let mut map_struct_purchase_period = LookupMap::new(b"m");
        map_struct_purchase_period.insert(&current,&self.get_user_purchases_in_given_period(&sender,&current));

        self.users.insert(&sender,&User {period_balance: map_struct,latest_period: current, purchases_in_period: map_struct_purchase_period});


        //add total for period
        self.optimize_total_balance(current);
        let total_balance_in_near = self.get_total_balance(&current).clone();
        let final_total_balance = deposit_amount + total_balance_in_near;
        self.total.insert(&current,&final_total_balance);
    }
    
    //withdraw, after optimization all deposited values are at latest period, so we can get balance of user them from there.
    
    #[payable]
    pub fn withdraw_balance(&mut self, amount: Balance){
        let sender: AccountId = env::signer_account_id();
        //user can only withdraw money not in current period current = currentblock- initblock / 37 days
        let current: u64 = (env::block_timestamp() - self.initial_stake_called_at.clone()) / WEEKANDMONTH ;
        
        self.optimize_user_balances(&sender,current);
        self.optimize_total_balance(current);
        //condition : withdraw week 1 month passed 
        if (env::block_timestamp() - WEEKANDMONTH * current) > MONTH {
            self.optimize_user_balances(&sender,current + 1);
            if amount <= (self.get_user_balance(&sender,&current) ){
                let user_balance = self.get_user_balance( &sender, &current).clone();
                let mut map_struct = LookupMap::new(b"m");
                let final_balance = user_balance - amount;
                map_struct.insert(&current,&final_balance);
                let mut map_struct_purchase_period = LookupMap::new(b"m");
        map_struct_purchase_period.insert(&current,&self.get_user_purchases_in_given_period(&sender,&current));

        self.users.insert(&sender,&User {period_balance: map_struct,latest_period: current, purchases_in_period: map_struct_purchase_period});

                self.optimize_total_balance(current);
                //remove from total balance
                let total_balance_in_near = self.get_total_balance(&current).clone();
                let final_total_balance = total_balance_in_near - amount;
                self.total.insert(&current,&final_total_balance);
                //TRANSFER HAPPENS HERE
                Promise::new(sender).transfer(amount);
            }
        }
        //condition : deposited while ongoing stake
        else{
            if amount <= self.get_user_balance(&sender,&current){
                let user_balance = self.get_user_balance( &sender, &current).clone();
                let mut map_struct = LookupMap::new(b"m");
                let final_balance = user_balance - amount;
                map_struct.insert(&current,&final_balance);
                let mut map_struct_purchase_period = LookupMap::new(b"m");
                map_struct_purchase_period.insert(&current,&self.get_user_purchases_in_given_period(&sender,&current));

                self.users.insert(&sender,&User {period_balance: map_struct,latest_period: current, purchases_in_period: map_struct_purchase_period});
                //TRANSFER HAPPENS HERE
                Promise::new(sender).transfer(amount);
            }
        }
        
        

        //condition : withdraw possible for a week after unstake
        //7 days in nano 604800000000000
        
    }
    

    

    
    //ONLY WORKS IN 37 DAY PERIOD
    #[payable]
    pub fn buy_launch_tokens(&mut self, token_address: AccountId){ 
        /*
        let sender: AccountId = env::signer_account_id();
        let current: u64 = (env::block_timestamp() - self.initial_stake_called_at.clone()) / WEEKANDMONTH ;
        self.optimize_user_balances(&sender,current);
        let purchasing_power = self.get_user_balance(&sender,&(current-1)); // purchase power
        self.optimize_total_balance(current);
        let total_purchasing_power = self.get_total_balance(&(current -1));
        let total_tokens = self.get_token_to_be_sold(&token_address);

        // percentage of tokens user can buy in this given period, User must not be able to call this function multiple times.
        let purhcase_amount = total_tokens * purchasing_power / total_purchasing_power;
        if self.get_user_purchases_in_given_period(&sender,&current) {

        }
        */
    }



    //GETTERS
    //view functions, free of charge
    */
    pub fn get_user_balance(&self,  user_account: &AccountId) -> Balance{
        match self.users.get(&user_account) {
            Some(value)  => {
                value
            },
            None => 0
        }
    }

    pub fn get_total_balance(&self) -> Balance{
        self.total
    }

    pub fn get_tokens(&self, account: &AccountId) ->Balance{
        match self.tokens.get(&account) {
            Some(value) => {
                value
            },
            None => 0
        }
    }

    /*
    pub fn get_user_purchases_in_given_period(&self,  user_account: &AccountId, period: &u64) -> bool{
        match self.users.get(&user_account) {
            Some(value)  => {
                match value.purchases_in_period.get(period) {
                    Some(val) =>{
                        val
                    }
                    None=> false
                }
            },
            None => false
        }
    }
    
    pub fn get_latest_period(&self,  user_account: &AccountId) -> u64{
        match self.users.get(&user_account) {
            Some(value)  => {
                value.latest_period.into()
            },
            None => 0
        }
    }

    pub fn get_total_balance(&self,  period: &u64) -> Balance{
        match self.total.get(period) {
            Some(value)  => {
                value.into()
            },
            None => 0
        }
    }

    pub fn get_token_to_be_sold(&self, token_address: &AccountId ) -> Balance{
        match self.tokens.get(token_address) {
            Some(value)  => {
                value.into()
            },
            None => 0
        }
    }



    // ADMIN FUNCTIONS

    //Stakes near tokens for 1 month, sets launch period as started.
    pub fn stake_near(&mut self){
        //requires that 1 month + 7 days has passed
        if env::block_timestamp() > (self.initial_stake_called_at.clone() + WEEKANDMONTH){
            

        } 
    }

    //can be called by everyone, but i will also call it.
    pub fn unstake_near(&mut self){
        //requires that 1 month has passed 
        if env::block_timestamp() > (self.initial_stake_called_at.clone() + WEEKANDMONTH){

        }
    }

    pub fn start_launch(&mut self, token_to_sell:  AccountId, total_token_amount: u128){
        let this_account = near_sdk::env::current_account_id();
        let message = "launch started";
        let tx_data = json!({  "receiver_id": this_account, "amount": total_token_amount, "msg": message  }).to_string().into_bytes();
        self.tokens.insert(&token_to_sell,&total_token_amount);
        Promise::new(token_to_sell).function_call("ft_transfer".to_string(),tx_data,NO_DEPOSIT,XCC_GAS);
    }
    */

}
