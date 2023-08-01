pub use crate::{
    impls::beta0_core::{
        beta0_core, data,
        data::{Manager, *},
        *,
    },
    traits::beta0_core::*,
};
// use ink::prelude::vec::Vec;
use openbrush::{
    contracts::{ownable, pausable},
    traits::Storage,
};

// Storage<Manager>

pub trait CoreImpl: Storage<pausable::Data> + Storage<ownable::Data> {
    // Set Function
    /// Set hash code
    fn set_code(&mut self, code_hash: [u8; 32]) -> Result<(), CoreError> {
        ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
            panic!(
                "Failed to `set_code_hash` to {:?} due to {:?}",
                code_hash, err
            )
        });
        ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
        Ok(())
    }
}
