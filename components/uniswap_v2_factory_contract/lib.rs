#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod uniswap_v2_factory{
    use ink_lang as ink;
    use ink_storage::{traits::SpreadAllocate, Mapping};
    // pub use swap_traits::{IUniswapV2Factory, UniswapFactoryError, UniswapFactoryResult};


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

        // #[ink(message)]
        // fn all_pairs_length(&mut self) -> u64;

        // #[ink(message)]
        // fn create_pair(&mut self, token_a: AccountId, token_b: AccountId) -> AccountId;

        #[ink(message)]
        fn set_fee_to(&mut self, address: AccountId);

        #[ink(message)]
        fn set_fee_to_setter(&mut self, address: AccountId);
    }



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
        
        // #[ink(message)]                                                 //-> UniswapFactoryResult<()>
        // fn create_pair(&mut self, token_a: AccountId, token_b: AccountId)  {
                // if token_a != token_b {
                //     // Conditional Operator (? :)
                //     // The conditional operator first evaluates an expression for a true or false value and then executes 
                //     // one of the two given statements depending upon the result of the evaluation.

                //     if token_b > token_a {
                //         let token0: AccountId = token_a;
                //         let token1: AccountId = token_b;
                //     }
                //     else {
                //         let token0: AccountId = token_b;
                //         let token1: AccountId = token_a;
                //     }
                    
                       
                    
                //     todo!();
                    
                // }
        //         todo!();
        // }
        // #[ink(message)]
        // fn all_pairs_length(&mut self)  {
        //     // return self.all_pairs.length;
        //     todo!();
        // }
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

    }

    
    
      
}