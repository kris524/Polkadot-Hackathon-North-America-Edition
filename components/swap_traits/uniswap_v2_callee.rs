//! A callback for informing others about a swap.
//!
//! Seems to be used for flash swaps.
//!
//! # References
//!
//! - <https://github.com/Uniswap/v2-core/blob/master/contracts/interfaces/IUniswapV2Callee.sol>
//! - <https://docs.uniswap.org/protocol/V2/guides/smart-contract-integration/using-flash-swaps>

use ink_env::AccountId;
use ink_lang as ink;
use ink_prelude::vec::Vec;

#[ink::trait_definition]
pub trait IUniswapV2Callee {
    #[ink(message)]
    fn uniswap_v2_call(&mut self, sender: AccountId, amount0: u64, amount1: u64, data: Vec<u8>);
}
