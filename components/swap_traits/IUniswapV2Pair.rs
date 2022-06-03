// Useful docs: https://docs.uniswap.org/protocol/V2/reference/smart-contracts/pair


//Big help from this: https://github.com/paritytech/ink/blob/master/examples/trait-erc20/lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::trait_definition]
pub trait IUniswapV2Pair {

    #[ink(event)]
    struct Approval{
        #[ink(topic)]
        owner: AccountId,

        #[ink(topic)]
        spender: AccountId,

        value: Balance,
    }

    #[ink(event)]
    struct Transfer{
        #[ink(topic)]
        from: AccountId,

        #[ink(topic)]
        to: AccountId,

        value: Balance,
    }

    #[ink(message)]
    pub fn name(&self) -> String; //I am supposed to return a (string memory) but I cant figure out what to use instead

    #[ink(message)]
    pub fn symbol(&self) -> String;

    #[ink(message)]
    pub fn decimals(&self) -> u8;

    #[ink(message)]
    pub fn totalSupply()


    



}