pub use crate::{
    impls::beta0_core::{beta0_core, data, data::*, *},
    traits::beta0_core::*,
};
use ink::{
    codegen::{EmitEvent, Env},
    storage::Mapping,
};
use openbrush::{
    contracts::{
        ownable, pausable,
        traits::{ownable::*, pausable::*, psp22::*},
    },
    traits::{AccountId, Storage, DefaultEnv},
};

pub trait CoreImpl:
    Storage<data::Manager> + Storage<pausable::Data> + Storage<ownable::Data>
{
    // fn play(&mut self, bet_number: u32, is_over: u8) -> Result<(), PSP22Error> {
    //     // state contract
    //     if pausable::Internal::_paused(self) {
    //         return Err(PSP22Error::Custom(String::from("P::Contract is paused")));
    //     }

    //     let player = <Self as DefaultEnv>::env().caller();
    //     let bet_amount = <Self as DefaultEnv>::env().transferred_value();
    //     let max_bet = (<Self as DefaultEnv>::env().balance())
    //         .checked_div(self.data::<data::Manager>().max_bet_ratio as u128)
    //         .unwrap();

    //     assert!((1..=max_bet).contains(&bet_amount));

    //     if is_over == 1 {
    //         assert!((self.min_over_number..=self.max_over_number).contains(&bet_number));
    //     } else if is_over == 0 {
    //         assert!((self.min_under_number..=self.max_under_number).contains(&bet_number));
    //     }

    //     let bet_info = self.manager.bets.get(&player);

    //     if let Some(_unwrapped_bet_info) = bet_info {
    //         return Err(PSP22Error::Custom(String::from("O::Bet Not Finalized")));
    //     }

    //     let new_bet = BetInformation {
    //         is_over,
    //         bet_number,
    //         bet_amount,
    //     };

    //     //Update listed token
    //     self.manager.bets.insert(&player, &new_bet);

    //     // self.env().emit_event(PlayEvent {
    //     //     player: Some(player),
    //     //     is_over,
    //     //     bet_number,
    //     //     bet_amount,
    //     // });

    //     Ok(())
    // }
}
