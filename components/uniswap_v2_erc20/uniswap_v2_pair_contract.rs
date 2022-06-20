#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod uniswap_v2_pair {
    use ink_env::AccountId;
    use ethereum_types;
    use uniswap_v2_erc20::{Math, SafeMath, UQ112x112};
    use uniswap_v2_erc20::uniswap_v2_erc20;
    use swap_traits::{IUniswapV2Pair, IUniswapV2Callee, IUniswapV2Factory, Erc20};
    
    const MINIMUM_LIQUIDITY: U256 = 10**3;
    //bytes4 private constant SELECTOR = bytes4(keccak256(bytes('transfer(address,uint256)')));
    
    fn get_reserve(_reserve_0: U112, _reserve_1: U112, _block_time_stamp_last: u32) {
        _reserve_0 = reserve_0;
        _reserve_1 = reserve_1;
        _block_time_stamp_last = block_time_stamp_last;
    }
}