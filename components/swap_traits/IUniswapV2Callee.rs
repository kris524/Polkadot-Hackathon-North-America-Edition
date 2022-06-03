
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// Source of information: https://docs.uniswap.org/protocol/V2/guides/smart-contract-integration/using-flash-swaps
#[ink::trait_definition]
pub trait IUniswapV2Callee {

    //All public functions must use this attribute
    #[ink(message, payable)]
    fn uniswapV2Call(sender: &AccountId , amount0: u256, amount1: u256, data: Vec<u32>); //What does this thing return???
}  