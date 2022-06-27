#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused)]

use ink_lang as ink;

#[ink::contract]
mod uniswap_v2_factory{
    use ink_lang as ink;
    use ink_storage::{traits::SpreadAllocate, Mapping};
    // pub use swap_traits::{IUniswapV2Factory, UniswapFactoryError, UniswapFactoryResult};
    use swap_traits::IUniswapV2Factory;

    #[derive(Default)]
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct UniswapV2Factory {
        fee_to: AccountId,
        fee_to_setter: AccountId,
        // get_pair: Mapping<AccountId, Mapping<AccountId, AccountId>>,
        // all_pairs: AccountId,
        fees: Balance

    }


    #[ink(event)]
    pub struct PairCreated {
        #[ink(topic)]
        token0: AccountId,
        #[ink(topic)]
        token1: AccountId,
        #[ink(topic)]
        pair: Balance
    }

    impl UniswapV2Factory {
        

        #[ink(constructor)]
        pub fn new(_fees: Balance) -> Self {
            // Sets fees to zero if not in valid range
            Self {
                fees: if _fees >= 1000 { 0 } else { _fees },
                ..Default::default()
            }
        }

    }
  
    

    impl IUniswapV2Factory for UniswapV2Factory {
        
        #[ink(message)]
        fn fee_to(&self) -> AccountId {
            todo!();
        }

        #[ink(message)]
        fn fee_to_setter(&self) -> AccountId {
            todo!();
        }

        #[ink(message)]
        fn get_pair(&self, toekn_a: AccountId, token_b: AccountId) -> AccountId{
            todo!();
        }

        #[ink(message)]
        fn all_pairs(&self, log_value: u64) -> AccountId {
            todo!();
        }

        #[ink(message)]
        fn all_pairs_length(&self) -> u64 {
            todo!()
        }

         #[ink(message)]
        fn create_pair(&mut self, token_a: AccountId, token_b: AccountId) -> AccountId {
            todo!()
        }

        #[ink(message)]
        fn set_fee_to(&mut self, _fee_to: AccountId) {
            let sender = self.env().caller();
            if sender == self.fee_to_setter {
                self.fee_to = _fee_to;
            }
        }
        #[ink(message)]
        fn set_fee_to_setter(&mut self, _fee_to_setter: AccountId) {
            let sender = self.env().caller();
            if sender == self.fee_to_setter {
                self.fee_to_setter = _fee_to_setter;
            }
        }
    }

}
