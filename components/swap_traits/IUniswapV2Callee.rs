
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
trait IUniswapV2Callee {
    fn uniswapV2Call( amount: u32, amount1: u32) //Will continue tommorow, find a address and bytes data types
}