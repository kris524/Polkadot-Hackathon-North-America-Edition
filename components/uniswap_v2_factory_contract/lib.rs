#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod uniswap_v2_factory{

    use ink_storage::{traits::SpreadAllocate, Mapping};

    // pub use swa
    pub use swap_traits::{IUniswapV2Factory, UniswapFactoryError, UniswapFactoryResult};
    // use swap_traits::{IUniswapV2Factory, UniswapFactoryError, UniswapFactoryResult};

    use ink_lang as ink;



    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct UniswapV2Factory {
        fee_to: AccountId,
        fee_to_setter: AccountId,
        get_pair: Mapping<AccountId, Mapping<AccountId, AccountId>>,
        all_pairs: AccountId,
        // balance: Balance

    }


    #[ink(event)]
    pub struct PairCreated {
        token0: AccountId,
        token1: AccountId,
        pair: Balance
    }

    impl UniswapV2Factory {

        #[ink(constructor)]
        pub fn new( _fee_to_setter: AccountId) -> Self{
                // self.fee_to_setter = self._fee_to_setter;
                todo!();
                ink_lang::utils::initialize_contract(|contract| {
                    Self::new_init(contract, _fee_to_setter)
                })
        }
  
    }

    impl IUniswapV2Factory for UniswapV2Factory {
        #[ink(message)]
        fn create_pair(&mut self, token_a: AccountId, token_b: AccountId) -> UniswapFactoryResult<()> {
                if token_a != token_b {
                    // Conditional Operator (? :)
                    // The conditional operator first evaluates an expression for a true or false value and then executes 
                    // one of the two given statements depending upon the result of the evaluation.

                    if token_b > token_a {
                        let token0: AccountId = token_a;
                        let token1: AccountId = token_b;
                    }
                    else {
                        let token0: AccountId = token_b;
                        let token1: AccountId = token_a;
                    }
                    
                       
                    
                    todo!();
                    
                }
        }
        #[ink(message)]
        fn all_pairs_length(&self) -> Balance {
            return self.all_pairs.length;
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
    
