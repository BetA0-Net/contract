#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Mintable, Ownable, Pausable)]
#[openbrush::contract]
pub mod bet_a0ref {
    use bet_a0::traits::core_token::*;
    use ink::storage::Mapping;
    use openbrush::{
        contracts::{
            ownable::{OwnableError, *},
            pausable::{PausableError, *},
            psp22::PSP22Error,
        },
        modifiers,
        traits::{Storage, String},
    };

    // Custom err
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => {
                    Error::Custom(String::from("O::CallerIsNotOwner"))
                }
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

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
        over_rates: Vec<u32>,
        under_rates: Vec<u32>,
        max_bet_ratio: u32,
        psp22: AccountId,
        token_ratio: u32,
        bets: Mapping<AccountId, BetInformation>,
        admin_account: AccountId,
        _reserved: Option<()>,
    }

    impl Default for Manager {
        fn default() -> Self {
            Self {
                over_rates: Default::default(),
                under_rates: Default::default(),
                max_bet_ratio: Default::default(),
                psp22: [0u8; 32].into(),
                token_ratio: Default::default(),
                bets: Default::default(),
                admin_account: [0u8; 32].into(),
                _reserved: Default::default(),
            }
        }
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct CoreTokenContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        pausable: pausable::Data,
        #[storage_field]
        manager: Manager,
        min_over_number: u32,
        max_over_number: u32,
        min_under_number: u32,
        max_under_number: u32,
    }

    impl CoreToken for CoreTokenContract {
        #[modifiers(only_owner)]
        #[ink(message)]
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            psp22::Internal::_mint_to(self, account, amount)
        }
    }

    #[ink(event)]
    pub struct WinEvent {
        player: Option<AccountId>,
        is_over: u8,
        random_number: u32,
        bet_number: u32,
        bet_amount: Balance,
        win_amount: Balance,
    }

    #[ink(event)]
    pub struct LoseEvent {
        player: Option<AccountId>,
        is_over: u8,
        random_number: u32,
        bet_number: u32,
        bet_amount: Balance,
    }

    #[ink(event)]
    pub struct PlayEvent {
        player: Option<AccountId>,
        is_over: u8,
        bet_number: u32,
        bet_amount: Balance,
    }

    impl CoreTokenContract {
        #[ink(constructor)]
        pub fn new(
            max_bet_ratio: u32,
            psp22: AccountId,
            token_ratio: u32,
            min_over_number: u32,
            max_over_number: u32,
            min_under_number: u32,
            max_under_number: u32,
            admin_account: AccountId,
        ) -> Self {
            let mut instance = Self::default();
            let caller = Self::env().caller();
            ownable::Internal::_init_with_owner(&mut instance, caller);
            instance
                .initialize(
                    max_bet_ratio,
                    psp22,
                    token_ratio,
                    min_over_number,
                    max_over_number,
                    min_under_number,
                    max_under_number,
                    admin_account,
                )
                .ok()
                .unwrap();
            instance
        }

        // EXECUTE FUNCTIONS
        /// Function changes state
        #[ink(message)]
        pub fn change_state(&mut self) -> Result<(), PausableError> {
            let caller = Self::env().caller();
            if let Some(owner) = Ownable::owner(self) {
                if caller != owner {
                    return Err(From::from(PausableError::Paused));
                }

                if pausable::Internal::_paused(self) {
                    pausable::Internal::_unpause(self)
                } else {
                    pausable::Internal::_pause(self)
                }
            } else {
                return Err(From::from(PausableError::Paused));
            }
        }

        /// Function init
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            max_bet_ratio: u32,
            psp22: AccountId,
            token_ratio: u32,
            min_over_number: u32,
            max_over_number: u32,
            min_under_number: u32,
            max_under_number: u32,
            admin_account: AccountId,
        ) -> Result<(), Error> {
            // Make sure the initial data can only be init once
            if self.manager.psp22 != [0u8; 32].into() {
                return Err(Error::Custom(String::from("Contract Already Init")));
            }
            self.manager.over_rates = [
                0, 0, 0, 0, 10368, 10478, 10591, 10706, 10824, 10944, 11067, 11193, 11321, 11453,
                11588, 11726, 11867, 12012, 12160, 12312, 12468, 12628, 12792, 12960, 13133, 13310,
                13493, 13680, 13873, 14071, 14275, 14485, 14701, 14924, 15153, 15390, 15634, 15887,
                16147, 16416, 16694, 16982, 17280, 17589, 17909, 18240, 18584, 18942, 19313, 19700,
                20102, 20520, 20957, 21413, 21888, 22386, 22906, 23452, 24024, 24625, 25256, 25921,
                26621, 27361, 28142, 28970, 29848, 30781, 31774, 32833, 33965, 35178, 36481, 37884,
                39400, 41041, 42826, 44772, 46904, 49250, 51842, 54722, 57941, 61562, 65666, 70357,
                75769, 82083, 89545, 98500, 109444, 123125, 140714, 164166, 197000, 246250, 328333,
                492500, 985000, 0,
            ]
            .to_vec();
            self.manager.under_rates = [
                0, 985000, 492500, 328333, 246250, 197000, 164166, 140714, 123125, 109444, 98500,
                89545, 82083, 75769, 70357, 65666, 61562, 57941, 54722, 51842, 49250, 46904, 44772,
                42826, 41041, 39400, 37884, 36481, 35178, 33965, 32833, 31774, 30781, 29848, 28970,
                28142, 27361, 26621, 25921, 25256, 24625, 24024, 23452, 22906, 22386, 21888, 21413,
                20957, 20520, 20102, 19700, 19313, 18942, 18584, 18240, 17909, 17589, 17280, 16982,
                16694, 16416, 16147, 15887, 15634, 15390, 15153, 14924, 14701, 14485, 14275, 14071,
                13873, 13680, 13493, 13310, 13133, 12960, 12792, 12628, 12468, 12312, 12160, 12012,
                11867, 11726, 11588, 11453, 11321, 11193, 11067, 10944, 10824, 10706, 10591, 10478,
                10368, 0, 0, 0, 0,
            ]
            .to_vec();
            self.manager.max_bet_ratio = max_bet_ratio;
            self.manager.psp22 = psp22;
            self.manager.token_ratio = token_ratio;
            self.min_over_number = min_over_number;
            self.max_over_number = max_over_number;
            self.min_under_number = min_under_number;
            self.max_under_number = max_under_number;
            self.manager.admin_account = admin_account;
            Ok(())
        }

        /// Function reward token
        #[ink(message)]
        pub fn reward_token(
            &mut self,
            player: AccountId,
            bet_amount: Balance,
        ) -> Result<(), Error> {
            // state contract
            if pausable::Internal::_paused(self) {
                return Err(Error::Custom(String::from("P::Contract is paused")));
            }

            let to_sent = bet_amount
                .checked_div(self.manager.token_ratio as u128)
                .unwrap();

            let contract_balance =
                CoreTokenRef::balance_of(&self.manager.psp22, Self::env().account_id());
            if contract_balance >= to_sent {
                assert!(
                    PSP22Ref::transfer(&self.manager.psp22, player, to_sent, Vec::<u8>::new())
                        .is_ok()
                );
            } else if contract_balance > 0 {
                assert!(PSP22Ref::transfer(
                    &self.manager.psp22,
                    player,
                    contract_balance,
                    Vec::<u8>::new()
                )
                .is_ok());
            }
            //PSP22Ref::mint(&mut self.manager.psp22,player,bet_amount/ (self.manager.token_ratio as u256));
            Ok(())
        }

        /// Finalize Bet
        #[ink(message)]
        pub fn finalize(&mut self, player: AccountId, random_number: u32) -> Result<(), Error> {
            // state contract
            if pausable::Internal::_paused(self) {
                return Err(Error::Custom(String::from("P::Contract is paused")));
            }

            let caller = self.env().caller();

            if caller != self.manager.admin_account {
                return Err(Error::Custom(String::from("O::Caller is not admin")));
            }

            let bet_info = self.manager.bets.get(&player);

            if let Some(unwrapped_bet_info) = bet_info {
                let bet_number = unwrapped_bet_info.bet_number.clone();
                let bet_amount = unwrapped_bet_info.bet_amount.clone();
                let is_over = unwrapped_bet_info.is_over.clone();

                self.manager.bets.remove(&player);

                if is_over == 1 {
                    assert!((self.min_over_number..=self.max_over_number).contains(&bet_number));
                    if random_number > bet_number {
                        //WIN
                        //How much to send to winner
                        let win_amount = (self.manager.over_rates[bet_number as usize] as Balance)
                            .checked_mul(bet_amount)
                            .unwrap()
                            .checked_div(10000)
                            .unwrap();
                        if win_amount.checked_sub(bet_amount) > Some(self.env().balance()) {
                            return Err(Error::Custom(String::from("O::Not Enough Balance")));
                        }
                        assert!(self.env().transfer(player, win_amount).is_ok());
                        self.env().emit_event(WinEvent {
                            player: Some(player),
                            is_over,
                            random_number,
                            bet_number,
                            bet_amount,
                            win_amount,
                        });
                    } else {
                        //LOSE
                        self.env().emit_event(LoseEvent {
                            player: Some(player),
                            is_over,
                            random_number,
                            bet_number,
                            bet_amount,
                        });
                    }
                } else if is_over == 0 {
                    assert!((self.min_under_number..=self.max_under_number).contains(&bet_number));
                    if random_number < bet_number {
                        //WIN
                        //How much to send to winner
                        let win_amount = (self.manager.under_rates[bet_number as usize] as Balance)
                            .checked_mul(bet_amount)
                            .unwrap()
                            .checked_div(10000)
                            .unwrap();
                        if win_amount.checked_sub(bet_amount) > Some(self.env().balance()) {
                            return Err(Error::Custom(String::from("O::Not Enough Balance")));
                        }
                        assert!(self.env().transfer(player, win_amount).is_ok());
                        self.env().emit_event(WinEvent {
                            player: Some(player),
                            is_over,
                            random_number,
                            bet_number,
                            bet_amount,
                            win_amount,
                        });
                    } else {
                        //LOSE
                        self.env().emit_event(LoseEvent {
                            player: Some(player),
                            is_over,
                            random_number,
                            bet_number,
                            bet_amount,
                        });
                    }
                } else {
                    return Err(Error::Custom(String::from("O::Invalid Input")));
                }

                assert!(self.reward_token(player, bet_amount).is_ok());

                // PSP22Ref::mint(&self.manager.psp22,player,bet_amount/(self.manager.token_ratio as u256));
                Ok(())
            } else {
                return Err(Error::Custom(String::from("O::Bet Not Exist")));
            }
        }

        /// Play
        #[ink(message)]
        #[ink(payable)]
        pub fn play(&mut self, bet_number: u32, is_over: u8) -> Result<(), Error> {
            // state contract
            if pausable::Internal::_paused(self) {
                return Err(Error::Custom(String::from("P::Contract is paused")));
            }

            let player = self.env().caller();
            let bet_amount = self.env().transferred_value();
            let max_bet = (self.env().balance())
                .checked_div(self.manager.max_bet_ratio as u128)
                .unwrap();

            assert!((1..=max_bet).contains(&bet_amount));

            if is_over == 1 {
                assert!((self.min_over_number..=self.max_over_number).contains(&bet_number));
            } else if is_over == 0 {
                assert!((self.min_under_number..=self.max_under_number).contains(&bet_number));
            }

            let bet_info = self.manager.bets.get(&player);

            if let Some(_unwrapped_bet_info) = bet_info {
                return Err(Error::Custom(String::from("O::Bet Not Finalized")));
            }

            let new_bet = BetInformation {
                is_over,
                bet_number,
                bet_amount,
            };

            //Update listed token
            self.manager.bets.insert(&player, &new_bet);

            self.env().emit_event(PlayEvent {
                player: Some(player),
                is_over,
                bet_number,
                bet_amount,
            });

            Ok(())
        }

        /// Withdraw Fees - only Owner
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn withdraw_fee(&mut self, value: Balance) -> Result<(), Error> {
            // state contract
            if pausable::Internal::_paused(self) {
                return Err(Error::Custom(String::from("P::Contract is paused")));
            }

            if value > self.env().balance() {
                return Err(Error::Custom(String::from("O::Not Enough Balance")));
            }
            assert!(self.env().transfer(self.env().caller(), value).is_ok());
            Ok(())
        }

        /// Withdraw Fees - only Owner
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn withdraw_token(&mut self, value: Balance) -> Result<(), Error> {
            // state contract
            if pausable::Internal::_paused(self) {
                return Err(Error::Custom(String::from("P::Contract is paused")));
            }

            if value > PSP22Ref::balance_of(&self.manager.psp22, Self::env().account_id()) {
                return Err(Error::Custom(String::from("O::Not Enough Balance")));
            }
            assert!(PSP22Ref::transfer(
                &self.manager.psp22,
                self.env().caller(),
                value,
                Vec::<u8>::new()
            )
            .is_ok());
            Ok(())
        }

        // SET FUNCTIONS
        /// Set code_hash
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_code(&mut self, code_hash: [u8; 32]) -> Result<(), Error> {
            ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });
            ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
            Ok(())
        }

        /// Set over_rates and discount rate - Only Owner 2 vectors same size
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn set_rates(
            &mut self,
            over_rates: Vec<u32>,
            under_rates: Vec<u32>,
        ) -> Result<(), Error> {
            assert!(over_rates.len() == under_rates.len());

            self.manager.over_rates = over_rates;
            self.manager.under_rates = under_rates;
            Ok(())
        }

        /// Set new psp22 address
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn set_psp22(&mut self, psp22: AccountId) -> Result<(), Error> {
            self.manager.psp22 = psp22;
            Ok(())
        }

        /// Set new token ratio
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn set_token_ratio(&mut self, token_ratio: u32) -> Result<(), Error> {
            self.manager.token_ratio = token_ratio;
            Ok(())
        }

        /// Set min number over roll
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn set_min_number_over_roll(&mut self, min_over_number: u32) -> Result<(), Error> {
            self.min_over_number = min_over_number;
            Ok(())
        }

        /// Set max number over roll
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn set_max_number_over_roll(&mut self, max_over_number: u32) -> Result<(), Error> {
            self.max_over_number = max_over_number;
            Ok(())
        }

        /// Set min number under roll
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn set_min_number_under_roll(&mut self, min_under_number: u32) -> Result<(), Error> {
            self.min_under_number = min_under_number;
            Ok(())
        }

        /// Set max number under roll
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn set_max_number_under_roll(&mut self, max_under_number: u32) -> Result<(), Error> {
            self.max_under_number = max_under_number;
            Ok(())
        }

        /// Setters
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn set_max_bet_ratio(&mut self, max_bet_ratio: u32) -> Result<(), Error> {
            self.manager.max_bet_ratio = max_bet_ratio;
            Ok(())
        }

        /// Set admin id
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn set_admin_account(&mut self, admin_account: AccountId) -> Result<(), Error> {
            self.manager.admin_account = admin_account;
            Ok(())
        }

        // GET FUNCTIONS
        /// Is bet exist
        #[ink(message)]
        pub fn is_bet_available(&self, player: AccountId) -> bool {
            let bet_info = self.manager.bets.get(&player);
            bet_info.is_some()
        }

        /// get admin id
        #[ink(message)]
        pub fn get_admin_account(&self) -> AccountId {
            self.manager.admin_account
        }

        /// Is bet exist
        #[ink(message)]
        pub fn get_bet(&self, player: AccountId) -> Option<BetInformation> {
            let bet_info = self.manager.bets.get(&player);
            if let Some(_unwrapped_bet_info) = bet_info {
                return Some(bet_info.unwrap());
            }
            return None;
        }

        /// get min number over roll
        #[ink(message)]
        pub fn get_min_number_over_roll(&self) -> u32 {
            self.min_over_number
        }

        /// get max number over roll
        #[ink(message)]
        pub fn get_max_number_over_roll(&self) -> u32 {
            self.max_over_number
        }

        /// get min number under roll
        #[ink(message)]
        pub fn get_min_number_under_roll(&self) -> u32 {
            self.min_under_number
        }

        /// get max number under roll
        #[ink(message)]
        pub fn get_max_number_under_roll(&self) -> u32 {
            self.max_under_number
        }

        /// get contract token balance
        #[ink(message)]
        pub fn get_token_balance(&self) -> Balance {
            PSP22Ref::balance_of(&self.manager.psp22, Self::env().account_id())
        }

        /// Get token ratio
        #[ink(message)]
        pub fn get_token_ratio(&self) -> u32 {
            self.manager.token_ratio
        }

        /// Get psp22 address
        #[ink(message)]
        pub fn psp22(&self) -> AccountId {
            self.manager.psp22
        }

        /// Get Over Rates
        #[ink(message)]
        pub fn get_over_rates(&self) -> Vec<u32> {
            self.manager.over_rates.clone()
        }

        /// Get Under Rates
        #[ink(message)]
        pub fn get_under_rates(&self) -> Vec<u32> {
            self.manager.under_rates.clone()
        }

        /// Get Max Bet
        #[ink(message)]
        pub fn get_max_bet_ratio(&self) -> u32 {
            self.manager.max_bet_ratio
        }

        #[ink(message)]
        pub fn get_max_bet(&self) -> u128 {
            (self.env().balance())
                .checked_div(self.manager.max_bet_ratio as u128)
                .unwrap()
        }
    }
}
