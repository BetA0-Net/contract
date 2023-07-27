use openbrush::contracts::traits::{ownable::*, pausable::*, psp22::*};

#[openbrush::wrapper]
pub type BetA0CoreRef = dyn PSP22 + BetA0Core;

#[openbrush::trait_definition]
pub trait BetA0Core: Ownable + Pausable {
    /// Play
    #[ink(message)]
    #[ink(payable)]
    fn play(&mut self, bet_number: u32, is_over: u8) -> Result<(), PSP22Error>;
}
