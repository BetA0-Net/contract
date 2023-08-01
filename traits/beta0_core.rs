// use ink::prelude::vec::Vec;
use openbrush::{
    contracts::traits::{ownable::*, pausable::*, psp22::*},
    modifiers,
    traits::String,
};

#[openbrush::wrapper]
pub type BetA0CoreRef = dyn PSP22 + BetA0Core;

#[openbrush::trait_definition]
pub trait BetA0Core: Ownable + Pausable {
    //Execute function
    /// Play
    #[ink(message)]
    #[ink(payable)]
    fn play(&mut self, bet_number: u32, is_over: u8) -> Result<(), PSP22Error>;

    // Set function
    /// setcode
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_code(&mut self, code_hash: [u8; 32]) -> Result<(), CoreError>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CoreError {
    Custom(String),
    PSP22Error(PSP22Error),
    PausableError(PausableError),
    OwnableError(OwnableError),
}

impl From<PausableError> for CoreError {
    fn from(access: PausableError) -> Self {
        CoreError::PausableError(access)
    }
}

impl From<OwnableError> for CoreError {
    fn from(access: OwnableError) -> Self {
        CoreError::OwnableError(access)
    }
}

impl From<PSP22Error> for CoreError {
    fn from(error: PSP22Error) -> Self {
        CoreError::PSP22Error(error)
    }
}
