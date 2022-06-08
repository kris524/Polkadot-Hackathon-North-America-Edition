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
    

}
