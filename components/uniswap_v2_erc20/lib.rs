#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod uniswap_v2_erc20 {
    use ink_storage::{traits::SpreadAllocate, Mapping};
    use swap_traits::{Erc20, Erc20Error, Erc20Result};

    const NAME: &'static str = "Uniswap V2";
    const SYMBOL: &'static str = "UNI-V2";
    const DECIMALS: u8 = 18;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct UniswapV2Erc20 {
        pub total_supply: Balance,
        pub balance_of: Mapping<AccountId, Balance>,
        pub allowance: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl UniswapV2Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            ink_lang::utils::initialize_contract(|contract| {
                Self::new_init(contract, initial_supply)
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        #[ink(message)]
        pub fn name(&self) -> String {
            NAME.to_string()
        }

        #[ink(message)]
        pub fn symbol(&self) -> String {
            SYMBOL.to_string()
        }

        #[ink(message)]
        pub fn decimal(&self) -> u8 {
            DECIMALS
        }

        fn new_init(&mut self, initial_supply: Balance) {
            let caller = Self::env().caller();
            self.balance_of.insert(&caller, &initial_supply);
            self.total_supply = initial_supply;
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });
        }

        fn mint(&mut self, to: &AccountId, value: Balance) -> Erc20Result<()> {
            if let Some(total_supply) = self.total_supply.checked_add(value) {
                self.total_supply = total_supply;
            } else {
                return Err(Erc20Error::BalanceOverflowOrUnderflow);
            }

            let to_balance = self.balance_of_impl(to);

            if let Some(to_balance) = to_balance.checked_add(value) {
                self.balance_of.insert(to, &(to_balance));
            } else {
                return Err(Erc20Error::BalanceOverflowOrUnderflow);
            }

            self.env().emit_event(Transfer {
                from: None,
                to: Some(*to),
                value,
            });

            Ok(())
        }

        fn burn(&mut self, from: &AccountId, value: Balance) -> Erc20Result<()> {
            if let Some(total_supply) = self.total_supply.checked_sub(value) {
                self.total_supply = total_supply;
            } else {
                return Err(Erc20Error::BalanceOverflowOrUnderflow);
            }

            let from_balance = self.balance_of_impl(from);

            if let Some(from_balance) = from_balance.checked_sub(value) {
                self.balance_of.insert(from, &(from_balance));
            } else {
                return Err(Erc20Error::BalanceOverflowOrUnderflow);
            }

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: None,
                value,
            });
            Ok(())
        }

        fn balance_of_impl(&self, owner: &AccountId) -> Balance {
            self.balance_of.get(owner).unwrap_or_default()
        }

        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowance.get((owner, spender)).unwrap_or_default()
        }

        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Erc20Result<()> {
            let from_balance = self.balance_of_impl(from);
            if from_balance < value {
                return Err(Erc20Error::InsufficientBalance);
            }

            let from_balance = self.balance_of_impl(from);

            if let Some(from_balance) = from_balance.checked_sub(value) {
                let to_balance = self.balance_of_impl(to);

                if let Some(to_balance) = to_balance.checked_add(value) {
                    self.balance_of.insert(to, &(to_balance));
                    self.balance_of.insert(from, &(from_balance));
                } else {
                    return Err(Erc20Error::BalanceOverflowOrUnderflow);
                }
            } else {
                return Err(Erc20Error::BalanceOverflowOrUnderflow);
            }

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });
            Ok(())
        }
    }

    impl Erc20 for UniswapV2Erc20 {
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_impl(&owner)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> Erc20Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Erc20Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance_impl(&from, &caller);
            if allowance < value {
                return Err(Erc20Error::InsufficientAllowance);
            }
            self.transfer_from_to(&from, &to, value)?;

            if let Some(allowance) = allowance.checked_sub(value) {
                self.allowance.insert((&from, &caller), &(allowance));
            } else {
                return Err(Erc20Error::BalanceOverflowOrUnderflow);
            }

            Ok(())
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Erc20Result<()> {
            let owner = self.env().caller();
            self.allowance.insert((&owner, &spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        todo!();
    }
}
