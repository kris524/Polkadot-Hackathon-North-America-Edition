//! Manages token pairs.
//!
//! # References
//!
//! - <https://github.com/Uniswap/v2-core/blob/master/contracts/interfaces/IUniswapV2Factory.sol>
//! - <https://docs.uniswap.org/protocol/V2/reference/smart-contracts/factory>

use ink_env::AccountId;
use ink_lang as ink;

// Note on access modifier (https://www.c-sharpcorner.com/article/variables-and-types-in-solidity/)

// View functions are read only functions and do not modify the state of the block chain.
// In other words if you want to read data from the block chain one can use view.
// Getter method are by default view functions.
// (https://cryptomarketpool.com/pure-and-view-in-solidity-smart-contracts/#:~:text=payable%20function%20modifiers.-,View,-functions%20are%20read)

// An ink! message with a &self receiver may only read state whereas an ink!
//  message with a &mut self receiver may mutate the contract’s storage.
// (https://paritytech.github.io/ink/ink_lang/attr.contract.html#:~:text=Note%3A-,An,-ink!%20message%20with)

//TODO: Add this inside the contract
//     #[ink(event)]
//     pub struct PairCreated {
//         #[ink(topic)]
//         token: Option<AccountId>,

//         #[ink(topic)]
//         token1: Option<AccountId>,

//         pair: Option<AccountId>,

//         log_value: u32 //1 for the first pair created, 2 for the second
//     }

pub enum UniswapFactoryError {
    IdenticalAddress,
    ZeroAddress,
    PairExists,
}

pub type UniswapFactoryResult<T> = core::result::Result<T, UniswapFactoryError>;


#[ink::trait_definition]
pub trait IUniswapV2Factory {
    #[ink(message)]
    fn fee_to(&self) -> AccountId;

    #[ink(message)]
    fn fee_to_setter(&self) -> AccountId;

    #[ink(message)]
    fn get_pair(&self, toekn_a: AccountId, token_b: AccountId) -> AccountId;

    #[ink(message)]
    fn all_pairs(&self, log_value: u64) -> AccountId;

    //#[ink(message)]
    //fn all_pairs_length(&self) -> u64;

    //#[ink(message)]
    //fn create_pair(&mut self, token_a: AccountId, token_b: AccountId) -> AccountId;

    #[ink(message)]
    fn set_fee_to(&mut self, address: AccountId);

    #[ink(message)]
    fn set_fee_to_setter(&mut self, address: AccountId);
}
