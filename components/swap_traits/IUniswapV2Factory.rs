#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// Note on access modifier (https://www.c-sharpcorner.com/article/variables-and-types-in-solidity/)
// Public
// The Public element can be inherited and can be accessed by external elements. All can access a public element. 
// External
// The External element can’t be inherited but it can be accessed by external elements


// View functions are read only functions and do not modify the state of the block chain. 
// In other words if you want to read data from the block chain one can use view. 
// Getter method are by default view functions.
// (https://cryptomarketpool.com/pure-and-view-in-solidity-smart-contracts/#:~:text=payable%20function%20modifiers.-,View,-functions%20are%20read)

// An ink! message with a &self receiver may only read state whereas an ink!
//  message with a &mut self receiver may mutate the contract’s storage. 
// (https://paritytech.github.io/ink/ink_lang/attr.contract.html#:~:text=Note%3A-,An,-ink!%20message%20with)
#[ink::trait_definition]
pub trait IUniswapV2Factory {

    //helpful docs: https://docs.uniswap.org/protocol/V2/reference/smart-contracts/factory
    #[ink(event)]
    struct PairCreated {
        #[ink(topic)]
        token: AccountId,

        #[ink(topic)]
        token1: AccountId,

        pair: AccountId,

        log_value: u32 //1 for the first pair created, 2 for the second
    }

    impl PairCreated {
        // Not sure at all if this should be here or if need to have PairCreated implementaiton
        // fn new(token1: AccountId, token0: AccountId, pair: AccountId) -> PairCreated {
        //     PairCreated { toekn0: token1, }
        // } 
        // I am guessing the equivalent is createPair

        #[ink(message)]
        pub fn feeTo(&self) -> AccountId;
        #[ink(message)]
        pub fn feeToSetter(&self) -> AccountId;

        #[ink(message)]
        pub fn getPair(toeknA: AccountId, tokenB: AccountId) -> pair;
        #[ink(message)]
        pub fn allPairs(log_value: u32) -> pair;
        #[ink(message)]
        pub fn allPairsLength() -> log_value;

        pub fn createPair(tokenA: AccountId, tokenB: AccountId) -> pair;

        #[ink(message)]
        pub fn satFeeTo(address: AccountId);

        #[ink(message)]
        pub fn setFeeToSetter(address: AccountId);

    }            

}
