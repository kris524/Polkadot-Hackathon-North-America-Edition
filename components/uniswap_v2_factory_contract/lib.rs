#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod uniswap_v2_factory {
    use ink_prelude::vec::Vec;
    use ink_storage::{traits::SpreadAllocate, Mapping};
    use swap_traits::IUniswapV2Factory;

    #[derive(Default)]
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct UniswapV2Factory {
        fee_to: AccountId,
        fee_to_setter: AccountId,
        /// This key tuple is ordered.
        /// Accessing it backward will always yield None.
        get_pair_map: Mapping<(AccountId, AccountId), AccountId>,
        all_pairs: Vec<AccountId>,
    }

    #[ink(event)]
    pub struct PairCreated {
        #[ink(topic)]
        token0: AccountId,
        #[ink(topic)]
        token1: AccountId,
        #[ink(topic)]
        pair: Balance,
    }

    impl UniswapV2Factory {
        #[ink(constructor)]
        pub fn new(fee_to_setter: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|this: &mut Self| this.new_init(fee_to_setter))
        }

        fn new_init(&mut self, fee_to_setter: AccountId) {
            self.fee_to_setter = fee_to_setter
        }
    }

    impl IUniswapV2Factory for UniswapV2Factory {
        #[ink(message)]
        fn fee_to(&self) -> AccountId {
            self.fee_to
        }

        #[ink(message)]
        fn fee_to_setter(&self) -> AccountId {
            self.fee_to_setter
        }

        #[ink(message)]
        fn get_pair(&self, token_a: AccountId, token_b: AccountId) -> Option<AccountId> {
            let (token_a, token_b) = if token_a < token_b {
                (token_a, token_b)
            } else {
                (token_b, token_a)
            };
            self.get_pair_map.get((token_a, token_b))
        }

        #[ink(message)]
        fn all_pairs(&self) -> Vec<AccountId> {
            self.all_pairs.clone()
        }

        #[ink(message)]
        fn all_pairs_length(&self) -> u64 {
            u64::try_from(self.all_pairs.len()).expect("overflow")
        }

        #[ink(message)]
        fn create_pair(&mut self, _token_a: AccountId, _token_b: AccountId) -> AccountId {
            todo!()
        }

        #[ink(message)]
        fn set_fee_to(&mut self, fee_to: AccountId) {
            let sender = self.env().caller();
            if sender == self.fee_to_setter {
                self.fee_to = fee_to;
            }
        }
        #[ink(message)]
        fn set_fee_to_setter(&mut self, fee_to_setter: AccountId) {
            let sender = self.env().caller();
            if sender == self.fee_to_setter {
                self.fee_to_setter = fee_to_setter;
            }
        }
    }
}
