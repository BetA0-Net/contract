use ink::prelude::vec::Vec;
use openbrush::{
    contracts::traits::{ownable::*, pausable::*, psp22::*},
    modifiers,
    traits::{AccountId, Balance, String},
};

#[openbrush::wrapper]
pub type BetA0CoreRef = dyn PSP22 + BetA0CoreTrait;

#[openbrush::trait_definition]
pub trait BetA0CoreTrait: Ownable + Pausable {
    //Execute function
    /// Function changes state
    #[ink(message)]
    fn change_state(&mut self) -> Result<(), PausableError>;

    /// tranfer token to pool
    #[ink(message)]
    #[modifiers(only_owner)]
    fn tranfer_token_to_pool(&mut self, pool: AccountId, amount: Balance) -> Result<(), CoreError>;

    /// reward token by bet pool
    #[ink(message)]
    fn reward_token_to_player(
        &mut self,
        player: AccountId,
        bet_amount: Balance,
    ) -> Result<(), CoreError>;

    /// Function reward token
    #[ink(message)]
    fn reward_token(&mut self, player: AccountId, bet_amount: Balance) -> Result<(), CoreError>;

    /// Withdraw Fees - only Owner
    #[ink(message)]
    #[modifiers(only_owner)]
    fn withdraw_fee(&mut self, value: Balance) -> Result<(), CoreError>;

    /// Withdraw Token - only Owner
    #[ink(message)]
    #[modifiers(only_owner)]
    fn withdraw_token(&mut self, value: Balance) -> Result<(), CoreError>;

    // Set function
    /// setcode
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_code(&mut self, code_hash: [u8; 32]) -> Result<(), CoreError>;

    /// Set min number over roll
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_min_number_over_roll(&mut self, min_over_number: u32) -> Result<(), CoreError>;

    /// Set max number over roll
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_max_number_over_roll(&mut self, max_over_number: u32) -> Result<(), CoreError>;

    /// Set min number under roll
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_min_number_under_roll(&mut self, min_under_number: u32) -> Result<(), CoreError>;

    /// Set max number under roll
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_max_number_under_roll(&mut self, max_under_number: u32) -> Result<(), CoreError>;

    /// Set over_rates and discount rate - Only Owner 2 vectors same size
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_rates(&mut self, over_rates: Vec<u32>, under_rates: Vec<u32>) -> Result<(), CoreError>;

    /// Set new psp22 address
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_bet_token_address(&mut self, bet_token_address: AccountId) -> Result<(), CoreError>;

    /// Set new token ratio
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_token_ratio(&mut self, token_ratio: u32) -> Result<(), CoreError>;

    /// Set max bet ratio
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_max_bet_ratio(&mut self, max_bet_ratio: u32) -> Result<(), CoreError>;

    /// Set revenue_ratio
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_revenue_ratio(&mut self, revenue_ratio: u32) -> Result<(), CoreError>;

    /// Set reward_pool
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_reward_pool(&mut self, reward_pool: AccountId) -> Result<(), CoreError>;

    /// Set max bet ratio
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_general_pool(&mut self, general_pool: AccountId) -> Result<(), CoreError>;

    /// Set bet_pool
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_bet_pool(&mut self, bet_pool: AccountId) -> Result<(), CoreError>;

    /// Set admin id
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_admin_account(&mut self, admin_account: AccountId) -> Result<(), CoreError>;

    // Get Function
    /// get min number over roll
    #[ink(message)]
    fn get_min_number_over_roll(&self) -> u32;

    /// get max number over roll
    #[ink(message)]
    fn get_max_number_over_roll(&self) -> u32;

    /// get min number under roll
    #[ink(message)]
    fn get_min_number_under_roll(&self) -> u32;

    /// get max number under roll
    #[ink(message)]
    fn get_max_number_under_roll(&self) -> u32;

    /// Get token ratio
    #[ink(message)]
    fn get_token_ratio(&self) -> u32;

    /// get revenue ratio
    #[ink(message)]
    fn get_revenue_ratio(&self) -> u32;

    /// get reward pool
    #[ink(message)]
    fn get_reward_pool(&self) -> AccountId;

    /// get general pool
    #[ink(message)]
    fn get_general_pool(&self) -> AccountId;

    /// get bet pool
    #[ink(message)]
    fn get_bet_pool(&self) -> AccountId;

    /// Get psp22 address
    #[ink(message)]
    fn bet_token_address(&self) -> AccountId;

    /// Get Over Rates
    #[ink(message)]
    fn get_over_rates(&self) -> Vec<u32>;

    /// Get Under Rates
    #[ink(message)]
    fn get_under_rates(&self) -> Vec<u32>;

    /// Get Max Bet
    #[ink(message)]
    fn get_max_bet_ratio(&self) -> u32;

    #[ink(message)]
    fn get_max_bet(&self) -> u128;

    /// get contract token balance
    #[ink(message)]
    fn get_token_balance(&self) -> Balance;

    /// get token balance pool
    #[ink(message)]
    fn get_token_balance_pool(&self, pool: AccountId) -> Balance;

    /// Is bet exist
    #[ink(message)]
    fn is_bet_available(&self, player: AccountId) -> bool;
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
