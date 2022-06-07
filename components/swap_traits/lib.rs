//! Translations of the Uniswap v2 interfaces to Ink traits.
//!
//! # References
//!
//! - <https://github.com/Uniswap/v2-core/blob/master/contracts/interfaces/>
//! - <https://docs.uniswap.org/protocol/V2/introduction>
//! - <https://docs.uniswap.org/protocol/V2/concepts/protocol-overview/smart-contracts>

#![cfg_attr(not(feature = "std"), no_std)]

mod uniswap_v2_callee;
mod uniswap_v2_factory;
mod uniswap_v2_pair;

pub use uniswap_v2_callee::*;
pub use uniswap_v2_factory::*;
pub use uniswap_v2_pair::*;
