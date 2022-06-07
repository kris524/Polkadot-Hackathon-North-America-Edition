
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_env::AccountId;

// Source of information: https://docs.uniswap.org/protocol/V2/guides/smart-contract-integration/using-flash-swaps
#[ink::trait_definition]
pub trait IUniswapV2Callee {

    //All public functions must use this attribute
    #[ink(message)]
    fn uniswap_v2_call(&self, sender: AccountId , amount0: u64, amount1: u64, data: Vec<u32>);
}  