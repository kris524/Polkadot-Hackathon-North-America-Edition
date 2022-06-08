use ink_env::AccountId;
use ink_lang as ink;

type Balance = <ink_env::DefaultEnvironment as ink_env::Environment>::Balance;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Erc20Error {
    InsufficientBalance,
    InsufficientAllowance,
    BalanceOverflowOrUnderflow,
}

pub type Erc20Result<T> = core::result::Result<T, Erc20Error>;

#[ink::trait_definition]
pub trait Erc20 {
    /// Returns the total token supply.
    #[ink(message)]
    fn total_supply(&self) -> Balance;

    /// Returns the account balance for the specified `owner`.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Balance;

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

    /// Transfers `value` amount of tokens from the caller's account to account `to`.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: Balance) -> Erc20Result<()>;

    /// Allows `spender` to withdraw from the caller's account multiple times, up to
    /// the `value` amount.
    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: Balance) -> Erc20Result<()>;

    /// Transfers `value` tokens on the behalf of `from` to the account `to`.
    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Erc20Result<()>;
}
