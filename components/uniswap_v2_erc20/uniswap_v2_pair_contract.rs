#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contrct]
mod uniswap_v2_pair {
    use ink_env::AccountId;
    use lib::uniswap_v2_erc20;
    use swap_traits::{IUniswapV2Pair, IUniswapV2Callee, IUniswapV2Factory, Erc20};

}