#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contrct]
mod uniswap_v2_factory{

    use ink_env::AccountId;
    use swap_traits::{IUniswapV2Factory};

    use ink_lang as ink;


    #[ink(storage)]
    pub struct UniswapV2Factory {
        fee_to: AccountId,
        fee_to_setter: AccountId,
        get_pair: Mapping<AccountId, Mapping<AccountId, AccountId>>,
        all_pairs: AccountId,

    }


    #[ink(event)]
    pub struct PairCreated {
        token0: AccountId,
        token1: AccountId,
        pair: Balance
    }

    impl UniswapV2Factory {

        #[ink(contstructor)]
        pub fn new(_fee_to_setter: AccountId) {
                fee_to_setter = _fee_to_setter;
        }
  
    }

    impl IUniswapV2Factory for UniswapV2Factory {
        fn create_pair(&mut self, token_a: AccountId, token_b: AccountId) -> AccountId {
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
                    if token0 != ??? && get_pair(token0, token1) == ??? {

                    }
                    
                }
        }

        fn all_pairs_length(&mut self) -> Balance {
            return all_pairs.length;
        }

        fn set_fee_to(&mut self, _fee_to: AccountId) {
            let sender = self.env().caller();
            if sender == fee_to_setter {
                fee_to = _fee_to;
            }
        }

        fn set_fee_to_setter(&mut self, _fee_to_setter: AccountId) {
            let sender = self.env().caller();
            if sender == fee_to_setter {
                fee_to_setter = _fee_to_setter;
            }
        }

    }

}
    
