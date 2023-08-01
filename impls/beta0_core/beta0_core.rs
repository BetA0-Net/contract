pub use crate::{
    impls::beta0_core::{
        beta0_core, data,
        data::{Manager, *},
        *,
    },
    traits::beta0_core::*,
};
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::{ownable::*, pausable::*, psp22::*},
    traits::{AccountId, Balance, Storage, String},
};

// Storage<data::Manager>

pub trait BetA0CoreTraitImpl:
    Storage<Manager>
    + Storage<pausable::Data>
    + Storage<ownable::Data>
    + pausable::Internal
    + ownable::Internal
    + ownable::Ownable
    + pausable::Pausable
{
    // Execute function
    /// Function changes state
    fn change_state(&mut self) -> Result<(), PausableError> {
        let caller = Self::env().caller();
        if let Some(owner) = Ownable::owner(self) {
            if caller != owner {
                return Err(From::from(PausableError::Paused));
            }

            if self._paused() {
                self._unpause()
            } else {
                self._pause()
            }
        } else {
            return Err(From::from(PausableError::Paused));
        }
    }

    /// tranfer token to pool
    fn tranfer_token_to_pool(&mut self, pool: AccountId, amount: Balance) -> Result<(), CoreError> {
        // state contract
        if pausable::Internal::_paused(self) {
            return Err(CoreError::Custom(String::from("P::Contract is paused")));
        }

        let contract_balance = PSP22Ref::balance_of(
            &self.data::<data::Manager>().bet_token_address,
            Self::env().account_id(),
        );

        if contract_balance > 0 {
            assert!(PSP22Ref::transfer(
                &self.data::<data::Manager>().bet_token_address,
                pool,
                amount,
                Vec::<u8>::new()
            )
            .is_ok());
        } else {
            return Err(CoreError::Custom(String::from("O::Not Enough Balance")));
        }

        Ok(())
    }

    /// reward token by bet pool
    fn reward_token_to_player(
        &mut self,
        player: AccountId,
        bet_amount: Balance,
    ) -> Result<(), CoreError> {
        // state contract
        if pausable::Internal::_paused(self) {
            return Err(CoreError::Custom(String::from("P::Contract is paused")));
        }

        let data_manager = self.data::<data::Manager>();

        let to_sent = bet_amount
            .checked_div(data_manager.token_ratio as u128)
            .unwrap();

        let pool_balance =
            PSP22Ref::balance_of(&data_manager.bet_token_address, data_manager.bet_pool);

        // ensure the user gave allowance to the contract
        if PSP22Ref::allowance(
            &data_manager.bet_token_address,
            data_manager.bet_pool,
            Self::env().account_id(),
        ) < to_sent
        {
            return Err(CoreError::Custom(String::from(
                "InsufficientAllowanceToLend",
            )));
        }

        if pool_balance >= to_sent {
            assert!(PSP22Ref::transfer_from(
                &data_manager.bet_token_address,
                data_manager.bet_pool,
                player,
                to_sent,
                Vec::<u8>::new()
            )
            .is_ok());
        } else if pool_balance > 0 {
            assert!(PSP22Ref::transfer_from(
                &data_manager.bet_token_address,
                data_manager.bet_pool,
                player,
                to_sent,
                Vec::<u8>::new()
            )
            .is_ok());
        }
        //PSP22Ref::mint(&mut self.manager.psp22,player,bet_amount/ (self.manager.token_ratio as u256));
        Ok(())
    }

    /// Function reward token
    fn reward_token(&mut self, player: AccountId, bet_amount: Balance) -> Result<(), CoreError> {
        // state contract
        if pausable::Internal::_paused(self) {
            return Err(CoreError::Custom(String::from("P::Contract is paused")));
        }

        let to_sent = bet_amount
            .checked_div(self.data::<data::Manager>().token_ratio as u128)
            .unwrap();

        let contract_balance = PSP22Ref::balance_of(
            &self.data::<data::Manager>().bet_token_address,
            Self::env().account_id(),
        );

        if contract_balance >= to_sent {
            assert!(PSP22Ref::transfer(
                &self.data::<data::Manager>().bet_token_address,
                player,
                to_sent,
                Vec::<u8>::new()
            )
            .is_ok());
        } else if contract_balance > 0 {
            assert!(PSP22Ref::transfer(
                &self.data::<data::Manager>().bet_token_address,
                player,
                contract_balance,
                Vec::<u8>::new()
            )
            .is_ok());
        }
        //PSP22Ref::mint(&mut self.manager.psp22,player,bet_amount/ (self.manager.token_ratio as u256));
        Ok(())
    }

    /// Withdraw Fees - only Owner
    fn withdraw_fee(&mut self, value: Balance) -> Result<(), CoreError> {
        // state contract
        if pausable::Internal::_paused(self) {
            return Err(CoreError::Custom(String::from("P::Contract is paused")));
        }

        if value > Self::env().balance() {
            return Err(CoreError::Custom(String::from("O::Not Enough Balance")));
        }
        assert!(Self::env().transfer(Self::env().caller(), value).is_ok());
        Ok(())
    }

    /// Withdraw Token - only Owner
    fn withdraw_token(&mut self, value: Balance) -> Result<(), CoreError> {
        // state contract
        if pausable::Internal::_paused(self) {
            return Err(CoreError::Custom(String::from("P::Contract is paused")));
        }

        if value
            > PSP22Ref::balance_of(
                &self.data::<data::Manager>().bet_token_address,
                Self::env().account_id(),
            )
        {
            return Err(CoreError::Custom(String::from("O::Not Enough Balance")));
        }
        assert!(PSP22Ref::transfer(
            &self.data::<data::Manager>().bet_token_address,
            Self::env().caller(),
            value,
            Vec::<u8>::new()
        )
        .is_ok());
        Ok(())
    }

    // Set Function
    /// Set min number over roll
    fn set_min_number_over_roll(&mut self, min_over_number: u32) -> Result<(), CoreError> {
        self.data::<data::Manager>().min_over_number = min_over_number;
        Ok(())
    }

    /// Set max number over roll
    fn set_max_number_over_roll(&mut self, max_over_number: u32) -> Result<(), CoreError> {
        self.data::<data::Manager>().max_over_number = max_over_number;
        Ok(())
    }

    /// Set min number under roll
    fn set_min_number_under_roll(&mut self, min_under_number: u32) -> Result<(), CoreError> {
        self.data::<data::Manager>().min_under_number = min_under_number;
        Ok(())
    }

    /// Set max number under roll
    fn set_max_number_under_roll(&mut self, max_under_number: u32) -> Result<(), CoreError> {
        self.data::<data::Manager>().max_under_number = max_under_number;
        Ok(())
    }

    /// Set over_rates and discount rate - Only Owner 2 vectors same size
    fn set_rates(&mut self, over_rates: Vec<u32>, under_rates: Vec<u32>) -> Result<(), CoreError> {
        assert!(over_rates.len() == under_rates.len());
        self.data::<data::Manager>().over_rates = over_rates;
        self.data::<data::Manager>().under_rates = under_rates;
        Ok(())
    }

    /// Set new psp22 address
    fn set_bet_token_address(&mut self, bet_token_address: AccountId) -> Result<(), CoreError> {
        self.data::<data::Manager>().bet_token_address = bet_token_address;
        Ok(())
    }

    /// Set new token ratio
    fn set_token_ratio(&mut self, token_ratio: u32) -> Result<(), CoreError> {
        self.data::<data::Manager>().token_ratio = token_ratio;
        Ok(())
    }

    /// Set max bet ratio
    fn set_max_bet_ratio(&mut self, max_bet_ratio: u32) -> Result<(), CoreError> {
        self.data::<data::Manager>().max_bet_ratio = max_bet_ratio;
        Ok(())
    }

    /// Set revenue_ratio
    fn set_revenue_ratio(&mut self, revenue_ratio: u32) -> Result<(), CoreError> {
        self.data::<data::Manager>().revenue_ratio = revenue_ratio;
        Ok(())
    }

    /// Set reward_pool
    fn set_reward_pool(&mut self, reward_pool: AccountId) -> Result<(), CoreError> {
        self.data::<data::Manager>().reward_pool = reward_pool;
        Ok(())
    }

    /// Set max bet ratio
    fn set_general_pool(&mut self, general_pool: AccountId) -> Result<(), CoreError> {
        self.data::<data::Manager>().general_pool = general_pool;
        Ok(())
    }

    /// Set bet_pool
    fn set_bet_pool(&mut self, bet_pool: AccountId) -> Result<(), CoreError> {
        self.data::<data::Manager>().bet_pool = bet_pool;
        Ok(())
    }

    /// Set admin id
    fn set_admin_account(&mut self, admin_account: AccountId) -> Result<(), CoreError> {
        self.data::<data::Manager>().admin_account = admin_account;
        Ok(())
    }

    // Get Function
    /// get min number over roll
    fn get_min_number_over_roll(&self) -> u32 {
        self.data::<data::Manager>().min_over_number
    }

    /// get max number over roll
    fn get_max_number_over_roll(&self) -> u32 {
        self.data::<data::Manager>().max_over_number
    }

    /// get min number under roll
    fn get_min_number_under_roll(&self) -> u32 {
        self.data::<data::Manager>().min_under_number
    }

    /// get max number under roll
    fn get_max_number_under_roll(&self) -> u32 {
        self.data::<data::Manager>().max_under_number
    }

    /// Get token ratio
    fn get_token_ratio(&self) -> u32 {
        self.data::<data::Manager>().token_ratio
    }

    /// get revenue ratio
    fn get_revenue_ratio(&self) -> u32 {
        self.data::<data::Manager>().revenue_ratio
    }

    /// get reward pool
    fn get_reward_pool(&self) -> AccountId {
        self.data::<data::Manager>().reward_pool
    }

    /// get general pool
    fn get_general_pool(&self) -> AccountId {
        self.data::<data::Manager>().general_pool
    }

    /// get bet pool
    fn get_bet_pool(&self) -> AccountId {
        self.data::<data::Manager>().bet_pool
    }

    /// Get psp22 address
    fn bet_token_address(&self) -> AccountId {
        self.data::<data::Manager>().bet_token_address
    }

    /// Get Over Rates
    fn get_over_rates(&self) -> Vec<u32> {
        self.data::<data::Manager>().over_rates.clone()
    }

    /// Get Under Rates
    fn get_under_rates(&self) -> Vec<u32> {
        self.data::<data::Manager>().under_rates.clone()
    }

    /// Get Max Bet
    fn get_max_bet_ratio(&self) -> u32 {
        self.data::<data::Manager>().max_bet_ratio
    }

    fn get_max_bet(&self) -> u128 {
        (Self::env().balance())
            .checked_div(self.data::<data::Manager>().max_bet_ratio as u128)
            .unwrap()
    }

    /// get contract token balance
    fn get_token_balance(&self) -> Balance {
        PSP22Ref::balance_of(
            &self.data::<data::Manager>().bet_token_address,
            Self::env().account_id(),
        )
    }

    /// get token balance pool
    fn get_token_balance_pool(&self, pool: AccountId) -> Balance {
        PSP22Ref::balance_of(&self.data::<data::Manager>().bet_token_address, pool)
    }

    /// Is bet exist
    fn is_bet_available(&self, player: AccountId) -> bool {
        let bet_info = self.data::<data::Manager>().bets.get(&player);
        bet_info.is_some()
    }
}
