// Useful docs: https://docs.uniswap.org/protocol/V2/reference/smart-contracts/pair


//Big help from this: https://github.com/paritytech/ink/blob/master/examples/trait-erc20/lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

use std::ops::Add;

use ink_lang as ink;
use ink_env::AccountId;

//Following a discussion on discord with the polkadot support team, they recommend I just use this: https://paritytech.github.io/ink/ink_prelude/string/struct.String.html
use ink_prelude::string::String;  


// #[ink(event)]
// struct Approval{
//     #[ink(topic)]
//     owner: AccountId,

//     #[ink(topic)]
//     spender: AccountId,

//     value: Balance,
// }

// #[ink(event)]
// struct Transfer{
//     #[ink(topic)]
//     from: AccountId,

//     #[ink(topic)]
//     to: AccountId,

//     value: Balance,
// }


#[ink::trait_definition]
pub trait IUniswapV2Pair {

  

    #[ink(message)]
    fn name(&self) -> String; 

    #[ink(message)]
    fn symbol(&self) -> String;

    #[ink(message)]
    fn decimals(&self) -> u8;

    #[ink(message)]
    //TODO: Use U256
    fn total_supply(&self) -> u64;

    #[ink(message)]
    //TODO: Use U256
    fn balance_of(&self, owner: AccountId) -> u64;

    #[ink(message)]
    //TODO: Use U256
    fn allowance(&self, owner: AccountId, spender: AccountId) -> u64;

    #[ink(message)]
    //TODO: Use U256
    fn approve(&self, spender: AccountId, value: u64) -> bool;

    #[ink(message)]
    //TODO: Use U256
    fn transfer(&self, to: AccountId, value: u64) -> bool;

    #[ink(message)]
    //TODO: Use U256
    fn transfer_from(&self, from: AccountId, to: AccountId, value: u64) -> bool;

    #[ink(message)]
    fn domain_separator(&self) -> Vec<u32>;
    #[ink(message)]
    fn permit_typehash(&self) -> Vec<u32>;

    #[ink(message)]
    //TODO: Use U256
    fn nonces(&self, owner: AccountId) -> u64;

    #[ink(message)]
    //TODO: Use U256
    fn permit(&self, owner: AccountId, spender: AccountId, value: u64, deadline: u64, v: u8, r: Vec<u32>, s: Vec<u32>);

    //event Mint, Burn, Swap, Sync

    #[ink(message)]
    //TODO: Use U256
    fn minimum_liquidity(&self) -> u64;

    #[ink(message)]
    fn factory(&self) -> AccountId;

    #[ink(message)]
    fn token0(&self) -> AccountId;

    #[ink(message)]
    fn token1(&self) -> AccountId;

//     function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
    #[ink(message)]
    fn get_reserves(&self) -> u64; // (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);

    #[ink(message)]
    fn price_0_cumulative_last(&self) -> u64;

    #[ink(message)]
    fn price_1_cumulative_last(&self) -> u64;

    #[ink(message)]
    fn k_last(&self) -> u64;

    #[ink(message)]
    fn mint(&self, to: AccountId) -> u64;

//     function burn(address to) external returns (uint amount0, uint amount1);
    #[ink(message)]
    fn burn(&self) -> u64; //(uint amount0, uint amount1);

//     function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external;
    #[ink(message)]
    fn swap(&self, amount0Out: u64,amount1Out: u64, to: AccountId, data: Vec<u32> ); // bytes calldata data

    #[ink(message)]
    fn skim(&self, to: AccountId); 

    #[ink(message)]
    fn sync(&self); 

//     function initialize(address, address) external;
    #[ink(message)]
    fn initialize(&self); //(address, address)







    



}