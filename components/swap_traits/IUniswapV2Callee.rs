
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::trait_definition]
pub trait IUniswapV2Callee {

    //All public functions must use this attribute
    #[ink(message)]
    fn uniswapV2Call(sender: &AccountId , amount0: &u32, amount1: &u32, data: &[i32]);
}