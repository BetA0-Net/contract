use openbrush::{
    contracts::traits::{
        ownable::*,
        pausable::*,
        psp22::{
            extensions::{burnable::*, mintable::*},
            *,
        },
    },
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type CoreTokenRef = dyn PSP22 + PSP22Mintable + Ownable + Pausable;

#[openbrush::trait_definition]
pub trait CoreToken: PSP22 + PSP22Mintable + Ownable + Pausable {
    #[ink(message)]
    fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}
