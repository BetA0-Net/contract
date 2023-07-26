use openbrush::{
    contracts::{
        psp22::Internal,
        traits::{
            ownable::*,
            pausable::*,
            psp22::{
                extensions::{burnable::*, metadata::*, mintable::*},
                *,
            },
        },
    },
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type BetTokenRef =
    dyn PSP22 + PSP22Metadata + PSP22Mintable + PSP22Burnable + Ownable + Pausable;

#[openbrush::trait_definition]
pub trait BetToken:
    PSP22 + PSP22Metadata + PSP22Mintable + PSP22Burnable + Ownable + Pausable + Internal
{
    #[ink(message)]
    fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}
