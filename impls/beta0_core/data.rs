use ink::storage::Mapping;
use openbrush::traits::{AccountId, Balance};

#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;

#[derive(
    Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Default, scale::Encode, scale::Decode,
)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct BetInformation {
    pub is_over: u8,
    pub bet_number: u32,
    pub bet_amount: Balance,
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

#[derive(Debug)]
#[openbrush::storage_item]
pub struct Manager {
    pub over_rates: Vec<u32>,
    pub under_rates: Vec<u32>,
    pub max_bet_ratio: u32,
    pub bet_token_address: AccountId,
    pub token_ratio: u32,
    pub bets: Mapping<AccountId, BetInformation>,
    pub admin_account: AccountId,
    pub _reserved: Option<()>,
}

impl Default for Manager {
    fn default() -> Self {
        Self {
            over_rates: Default::default(),
            under_rates: Default::default(),
            max_bet_ratio: Default::default(),
            bet_token_address: [0u8; 32].into(),
            token_ratio: Default::default(),
            bets: Default::default(),
            admin_account: [0u8; 32].into(),
            _reserved: Default::default(),
        }
    }
}
