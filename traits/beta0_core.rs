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
pub type CoreRef = dyn BetA0Core;

#[openbrush::trait_definition]
pub trait BetA0Core: Ownable + Pausable {
}
