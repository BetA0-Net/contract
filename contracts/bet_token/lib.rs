#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Metadata, PSP22Mintable, PSP22Burnable, Ownable, Pausable)]
#[openbrush::contract]
pub mod bet_token {
    use bet_a0::traits::bet_token::*;
    use openbrush::{
        contracts::{
            ownable::{OwnableError, *},
            pausable::{PausableError, *},
            psp22::PSP22Error,
        },
        modifiers,
        traits::{Storage, String},
    };

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

    #[derive(Debug)]
    #[openbrush::storage_item]
    pub struct Data {
        minter: AccountId,
    }

    impl Default for Data {
        fn default() -> Self {
            Self {
                minter: [0u8; 32].into(),
            }
        }
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct BetTokenContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        pausable: pausable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        data: Data,
    }

    impl BetToken for BetTokenContract {
        #[modifiers(only_owner)]
        #[ink(message)]
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            psp22::Internal::_burn_from(self, account, amount)
        }

        #[modifiers(only_owner)]
        #[ink(message)]
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            psp22::Internal::_mint_to(self, account, amount)
        }
    }

    impl BetTokenContract {
        #[ink(constructor)]
        pub fn new(
            initial_supply: Balance,
            minter: AccountId,
            name: Option<String>,
            symbol: Option<String>,
            decimal: u8,
        ) -> Self {
            let mut instance = Self::default();
            let caller = Self::env().caller();
            ownable::Internal::_init_with_owner(&mut instance, caller);
            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&decimal);
            instance.data.minter = minter;
            assert!(instance.mint(minter, initial_supply).is_ok());
            instance
        }

        // EXECUTE FUNCTIONS
        /// Function changes state
        #[ink(message)]
        pub fn change_state(&mut self) -> Result<(), PausableError> {
            let caller = Self::env().caller();
            if let Some(owner) = Ownable::owner(self) {
                if caller != owner {
                    return Err(From::from(PausableError::Paused));
                }

                if pausable::Internal::_paused(self) {
                    pausable::Internal::_unpause(self)
                } else {
                    pausable::Internal::_pause(self)
                }
            } else {
                return Err(From::from(PausableError::Paused));
            }
        }

        /// Only minter can mint
        #[ink(message)]
        pub fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            // state contract
            if pausable::Internal::_paused(self) {
                return Err(PSP22Error::Custom(String::from("P::Contract is paused")));
            }

            if self.data.minter != account {
                return Err(PSP22Error::Custom(String::from("Only minter can mint")));
            }
            psp22::Internal::_mint_to(self, account, amount)
        }

        /// Withdraw any Balance of Contract - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw(&mut self, value: Balance) -> Result<(), PSP22Error> {
            // state contract
            if pausable::Internal::_paused(self) {
                return Err(PSP22Error::Custom(String::from("P::Contract is paused")));
            }

            let caller = Self::env().caller();
            if value > self.env().balance() {
                return Err(PSP22Error::Custom(
                    String::from("Not enough balance").into(),
                ));
            }
            if self.env().transfer(caller, value).is_err() {
                panic!("error withdraw_fee")
            }
            Ok(())
        }

        // GET FUNCTIONS
        /// Get minter
        #[ink(message)]
        pub fn get_minter(&self) -> AccountId {
            self.data.minter
        }

        // SET FUNCTIONS
        /// Set minter
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn set_minter(&mut self, minter: AccountId) -> Result<(), PSP22Error> {
            self.data.minter = minter;
            Ok(())
        }
    }
}
