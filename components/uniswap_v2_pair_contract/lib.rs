#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_env::AccountId;

#[ink::trait_definition]
pub trait IUniswapV2Pair {
    #[ink(message)]
    fn name(&self) -> String;

    #[ink(message)]
    fn symbol(&self) -> String;

    #[ink(message)]
    fn decimals(&self) -> u8;

    #[ink(message)]
    fn total_supply(&self) -> u64;

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u64;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> u64;

    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: u64) -> bool;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: u64) -> bool;

    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: u64) -> bool;

    #[ink(message)]
    fn domain_separator(&self) -> Vec<u8>;

    #[ink(message)]
    fn permit_typehash(&self) -> Vec<u8>;

    #[ink(message)]
    fn nonces(&self, owner: AccountId) -> u64;

    // function permit(address owner, address spender, uint value, uint deadline, uint8 v, bytes32 r, bytes32 s) external;
    #[ink(message)]
    fn permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        value: u64,
        deadline: u64,
        v: u8,
        r: Vec<u8>,
        s: Vec<u8>,
    );

    //event Mint, Burn, Swap, Sync

    #[ink(message)]
    fn minimum_liquidity(&self) -> u64;

    #[ink(message)]
    fn factory(&self) -> AccountId;

    #[ink(message)]
    fn token0(&self) -> AccountId;

    #[ink(message)]
    fn token1(&self) -> AccountId;

    // function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
    #[ink(message)]
    fn get_reserves(&self) -> (u64, u64, u32);

    #[ink(message)]
    fn price_0_cumulative_last(&self) -> u64;

    #[ink(message)]
    fn price_1_cumulative_last(&self) -> u64;

    #[ink(message)]
    fn k_last(&self) -> u64;

    #[ink(message)]
    fn mint(&mut self, to: AccountId) -> u64;

    // function burn(address to) external returns (uint amount0, uint amount1);
    #[ink(message)]
    fn burn(&mut self) -> (u64, u64);

    // function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external;
    #[ink(message)]
    fn swap(&mut self, amount0_out: u64, amount1_out: u64, to: AccountId, data: Vec<u8>);

    #[ink(message)]
    fn skim(&mut self, to: AccountId);

    #[ink(message)]
    fn sync(&mut self);

    // function initialize(address, address) external;
    #[ink(message)]
    fn initialize(&mut self, address1: AccountId, address2: AccountId);
}



#[ink::contract]
pub mod uniswap_v2_pair_contract {
    use super::IUniswapV2Pair;
    const MINIMUM_LIQUIDITY: u64 = 10_u64.pow(3);


    #[ink(storage)]
    pub struct UniswapV2PairContract {
        factory: AccountId,
        token0: AccountId,
        token1: AccountId,
        reserve0: u64,
        reserve1: u64,
        block_time_stamp_last: u32,
        price_0_cumulative_last: u32,
        price_1_cumulative_last: u32,
        k_last: u32,
        unlocked: u32
    }

   

    impl UniswapV2PairContract {
        //constructor goes here
        #[ink(constructor)]
        pub fn new() -> Self {
            Self{
            factory: Default::default(),
            token0: Default::default(),
            token1: Default::default(),
            reserve0: Default::default(),
            reserve1: Default::default(),
            block_time_stamp_last: Default::default(),
            price_0_cumulative_last: Default::default(),
            price_1_cumulative_last: Default::default(),
            k_last: Default::default(),
            unlocked: Default::default(),
            }

        }
        
        
    }

    impl  IUniswapV2Pair for  UniswapV2PairContract {
        
        #[ink(message)]
        fn get_reserves(&self) -> (u64, u64, u32){
            todo!();
        }
        #[ink(message)]
        fn name(&self) -> String{
            todo!();
        }

        #[ink(message)]
        fn symbol(&self) -> String{
            todo!();
        }

        #[ink(message)]
        fn decimals(&self) -> u8{
            todo!();
        }

        #[ink(message)]
        fn total_supply(&self) -> u64{
            todo!();
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u64{
            todo!();
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> u64{
            todo!();
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: u64) -> bool{
            todo!();
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: u64) -> bool{
            todo!();
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: u64) -> bool{
            todo!();
        }


        #[ink(message)]
        fn domain_separator(&self) -> Vec<u8>{
            todo!();
        }

        #[ink(message)]
        fn permit_typehash(&self) -> Vec<u8>{
            todo!();
        }

        #[ink(message)]
        fn nonces(&self, owner: AccountId) -> u64{
            todo!();
        }

        #[ink(message)]
    fn permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        value: u64,
        deadline: u64,
        v: u8,
        r: Vec<u8>,
        s: Vec<u8>,
    ){
        todo!();
    }

    //event Mint, Burn, Swap, Sync

    #[ink(message)]
    fn minimum_liquidity(&self) -> u64{
        todo!();
    }

    #[ink(message)]
    fn factory(&self) -> AccountId{
        todo!();
    }

    #[ink(message)]
    fn token0(&self) -> AccountId{
        todo!();
    }

    #[ink(message)]
    fn token1(&self) -> AccountId{
        todo!();
    }

    // function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);

    #[ink(message)]
    fn price_0_cumulative_last(&self) -> u64{
        todo!();
    }

    #[ink(message)]
    fn price_1_cumulative_last(&self) -> u64{
        todo!();
    }

    #[ink(message)]
    fn k_last(&self) -> u64{
        todo!();
    }

    #[ink(message)]
    fn mint(&mut self, to: AccountId) -> u64{
        todo!();
    }

    // function burn(address to) external returns (uint amount0, uint amount1);
    #[ink(message)]
    fn burn(&mut self) -> (u64, u64){
        todo!();
    }

    // function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external;
    #[ink(message)]
    fn swap(&mut self, amount0_out: u64, amount1_out: u64, to: AccountId, data: Vec<u8>){
        todo!();
    }

    #[ink(message)]
    fn skim(&mut self, to: AccountId){
        todo!();
    }

    #[ink(message)]
    fn sync(&mut self){
        todo!();
    }

    // function initialize(address, address) external;
    #[ink(message)]
    fn initialize(&mut self, address1: AccountId, address2: AccountId){
        todo!();
    }

        
    }

    
}

