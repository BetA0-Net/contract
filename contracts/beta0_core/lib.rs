#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable, Pausable, Upgradeable)]
#[openbrush::contract]
pub mod beta0_core {
    use bet_a0::impls::beta0_core::{
        data::Manager, BetA0CoreTraitImpl, BetInformation, CoreError, *,
    };
    // use ink::codegen::{EmitEvent, Env};
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::{
            ownable::{OwnableError, *},
            pausable::{PausableError, *},
            psp22::PSP22Error,
        },
        modifiers,
        traits::{DefaultEnv, Storage, String},
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct BetA0CoreContract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        pausable: pausable::Data,
        #[storage_field]
        manager: Manager,
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

    impl BetA0CoreTraitImpl for BetA0CoreContract {}

    impl BetA0CoreTrait for BetA0CoreContract {
        // Execute function
        /// Function changes state
        #[ink(message)]
        fn change_state(&mut self) -> Result<(), PausableError> {
            BetA0CoreTraitImpl::change_state(self)
        }

        /// tranfer token to pool
        #[ink(message)]
        #[modifiers(only_owner)]
        fn tranfer_token_to_pool(
            &mut self,
            pool: AccountId,
            amount: Balance,
        ) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::tranfer_token_to_pool(self, pool, amount)
        }

        /// reward token by bet pool
        #[ink(message)]
        fn reward_token_to_player(
            &mut self,
            player: AccountId,
            bet_amount: Balance,
        ) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::reward_token_to_player(self, player, bet_amount)
        }

        /// Function reward token
        #[ink(message)]
        fn reward_token(
            &mut self,
            player: AccountId,
            bet_amount: Balance,
        ) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::reward_token(self, player, bet_amount)
        }

        /// Withdraw Fees - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        fn withdraw_fee(&mut self, value: Balance) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::withdraw_fee(self, value)
        }

        /// Withdraw Token - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        fn withdraw_token(&mut self, value: Balance) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::withdraw_token(self, value)
        }

        // Set function
        /// Set min number over roll
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_min_number_over_roll(&mut self, min_over_number: u32) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_min_number_over_roll(self, min_over_number)
        }

        /// Set max number over roll
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_max_number_over_roll(&mut self, max_over_number: u32) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_max_number_over_roll(self, max_over_number)
        }

        /// Set min number under roll
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_min_number_under_roll(&mut self, min_under_number: u32) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_min_number_under_roll(self, min_under_number)
        }

        /// Set max number under roll
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_max_number_under_roll(&mut self, max_under_number: u32) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_max_number_under_roll(self, max_under_number)
        }

        /// Set over_rates and discount rate - Only Owner 2 vectors same size
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_rates(
            &mut self,
            over_rates: Vec<u32>,
            under_rates: Vec<u32>,
        ) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_rates(self, over_rates, under_rates)
        }

        /// Set new psp22 address
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_bet_token_address(&mut self, bet_token_address: AccountId) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_bet_token_address(self, bet_token_address)
        }

        /// Set new token ratio
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_token_ratio(&mut self, token_ratio: u32) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_token_ratio(self, token_ratio)
        }

        /// Set max bet ratio
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_max_bet_ratio(&mut self, max_bet_ratio: u32) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_max_bet_ratio(self, max_bet_ratio)
        }

        /// Set revenue_ratio
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_revenue_ratio(&mut self, revenue_ratio: u32) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_revenue_ratio(self, revenue_ratio)
        }

        /// Set reward_pool
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_reward_pool(&mut self, reward_pool: AccountId) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_reward_pool(self, reward_pool)
        }

        /// Set max bet ratio
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_general_pool(&mut self, general_pool: AccountId) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_general_pool(self, general_pool)
        }

        /// Set bet_pool
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_bet_pool(&mut self, bet_pool: AccountId) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_bet_pool(self, bet_pool)
        }

        /// Set admin id
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_admin_account(&mut self, admin_account: AccountId) -> Result<(), CoreError> {
            BetA0CoreTraitImpl::set_admin_account(self, admin_account)
        }

        // Get Function
        /// get min number over roll
        #[ink(message)]
        fn get_min_number_over_roll(&self) -> u32 {
            BetA0CoreTraitImpl::get_min_number_over_roll(self)
        }

        /// get max number over roll
        #[ink(message)]
        fn get_max_number_over_roll(&self) -> u32 {
            BetA0CoreTraitImpl::get_max_number_over_roll(self)
        }

        /// get min number under roll
        #[ink(message)]
        fn get_min_number_under_roll(&self) -> u32 {
            BetA0CoreTraitImpl::get_min_number_under_roll(self)
        }

        /// get max number under roll
        #[ink(message)]
        fn get_max_number_under_roll(&self) -> u32 {
            BetA0CoreTraitImpl::get_max_number_under_roll(self)
        }

        /// Get token ratio
        #[ink(message)]
        fn get_token_ratio(&self) -> u32 {
            BetA0CoreTraitImpl::get_token_ratio(self)
        }

        /// get revenue ratio
        #[ink(message)]
        fn get_revenue_ratio(&self) -> u32 {
            BetA0CoreTraitImpl::get_token_ratio(self)
        }

        /// get reward pool
        #[ink(message)]
        fn get_reward_pool(&self) -> AccountId {
            BetA0CoreTraitImpl::get_reward_pool(self)
        }

        /// get general pool
        #[ink(message)]
        fn get_general_pool(&self) -> AccountId {
            BetA0CoreTraitImpl::get_general_pool(self)
        }

        /// get bet pool
        #[ink(message)]
        fn get_bet_pool(&self) -> AccountId {
            BetA0CoreTraitImpl::get_bet_pool(self)
        }

        /// Get psp22 address
        #[ink(message)]
        fn bet_token_address(&self) -> AccountId {
            BetA0CoreTraitImpl::bet_token_address(self)
        }

        /// Get Over Rates
        #[ink(message)]
        fn get_over_rates(&self) -> Vec<u32> {
            BetA0CoreTraitImpl::get_over_rates(self)
        }

        /// Get Under Rates
        #[ink(message)]
        fn get_under_rates(&self) -> Vec<u32> {
            BetA0CoreTraitImpl::get_under_rates(self)
        }

        /// Get Max Bet
        #[ink(message)]
        fn get_max_bet_ratio(&self) -> u32 {
            BetA0CoreTraitImpl::get_max_bet_ratio(self)
        }

        #[ink(message)]
        fn get_max_bet(&self) -> u128 {
            BetA0CoreTraitImpl::get_max_bet(self)
        }

        /// get contract token balance
        #[ink(message)]
        fn get_token_balance(&self) -> Balance {
            BetA0CoreTraitImpl::get_token_balance(self)
        }

        /// get token balance pool
        #[ink(message)]
        fn get_token_balance_pool(&self, pool: AccountId) -> Balance {
            BetA0CoreTraitImpl::get_token_balance_pool(self, pool)
        }

        /// Is bet exist
        #[ink(message)]
        fn is_bet_available(&self, player: AccountId) -> bool {
            BetA0CoreTraitImpl::is_bet_available(self, player)
        }

        /// get admin id
        #[ink(message)]
        fn get_admin_account(&self) -> AccountId {
            BetA0CoreTraitImpl::get_admin_account(self)
        }

        /// get bet
        #[ink(message)]
        fn get_bet(&self, player: AccountId) -> Option<BetInformation> {
            BetA0CoreTraitImpl::get_bet(self, player)
        }
    }

    impl BetA0CoreContract {
        #[ink(constructor)]
        pub fn new(
            max_bet_ratio: u32,
            revenue_ratio: u32,
            reward_pool: AccountId,
            general_pool: AccountId,
            bet_pool: AccountId,
            bet_token_address: AccountId,
            token_ratio: u32,
            min_over_number: u32,
            max_over_number: u32,
            min_under_number: u32,
            max_under_number: u32,
            admin_account: AccountId,
        ) -> Self {
            let mut instance = Self::default();
            let caller = <Self as DefaultEnv>::env().caller();
            ownable::Internal::_init_with_owner(&mut instance, caller);
            instance
                .initialize(
                    max_bet_ratio,
                    revenue_ratio,
                    reward_pool,
                    general_pool,
                    bet_pool,
                    bet_token_address,
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
        /// Function init
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            max_bet_ratio: u32,
            revenue_ratio: u32,
            reward_pool: AccountId,
            general_pool: AccountId,
            bet_pool: AccountId,
            bet_token_address: AccountId,
            token_ratio: u32,
            min_over_number: u32,
            max_over_number: u32,
            min_under_number: u32,
            max_under_number: u32,
            admin_account: AccountId,
        ) -> Result<(), CoreError> {
            // Make sure the initial data can only be init once
            if self.manager.bet_token_address != [0u8; 32].into() {
                return Err(CoreError::Custom(String::from("Contract Already Init")));
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
            assert!((1..=1000).contains(&revenue_ratio));
            self.manager.reward_pool = reward_pool;
            self.manager.general_pool = general_pool;
            self.manager.bet_pool = bet_pool;
            self.manager.revenue_ratio = revenue_ratio;
            self.manager.bet_token_address = bet_token_address;
            self.manager.token_ratio = token_ratio;
            self.manager.min_over_number = min_over_number;
            self.manager.max_over_number = max_over_number;
            self.manager.min_under_number = min_under_number;
            self.manager.max_under_number = max_under_number;
            self.manager.admin_account = admin_account;
            Ok(())
        }

        /// Play
        #[ink(message)]
        #[ink(payable)]
        pub fn play(&mut self, bet_number: u32, is_over: u8) -> Result<(), PSP22Error> {
            // state contract
            if pausable::Internal::_paused(self) {
                return Err(PSP22Error::Custom(String::from("P::Contract is paused")));
            }

            let player = self.env().caller();
            let bet_amount = self.env().transferred_value();
            let max_bet = (self.env().balance())
                .checked_div(self.manager.max_bet_ratio as u128)
                .unwrap();

            assert!((1..=max_bet).contains(&bet_amount));

            if is_over == 1 {
                assert!(
                    (self.manager.min_over_number..=self.manager.max_over_number)
                        .contains(&bet_number)
                );
            } else if is_over == 0 {
                assert!(
                    (self.manager.min_under_number..=self.manager.max_under_number)
                        .contains(&bet_number)
                );
            }

            let bet_info = self.manager.bets.get(&player);

            if let Some(_unwrapped_bet_info) = bet_info {
                return Err(PSP22Error::Custom(String::from("O::Bet Not Finalized")));
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

        /// Finalize Bet
        #[ink(message)]
        pub fn finalize(&mut self, player: AccountId, random_number: u32) -> Result<(), CoreError> {
            // state contract
            if pausable::Internal::_paused(self) {
                return Err(CoreError::Custom(String::from("P::Contract is paused")));
            }

            let caller = self.env().caller();

            if caller != self.manager.admin_account {
                return Err(CoreError::Custom(String::from("O::Caller is not admin")));
            }

            let bet_info = self.manager.bets.get(&player);

            if let Some(unwrapped_bet_info) = bet_info {
                let bet_number = unwrapped_bet_info.bet_number.clone();
                let bet_amount = unwrapped_bet_info.bet_amount.clone();
                let is_over = unwrapped_bet_info.is_over.clone();

                self.manager.bets.remove(&player);

                if is_over == 1 {
                    assert!(
                        (self.manager.min_over_number..=self.manager.max_over_number)
                            .contains(&bet_number)
                    );
                    if random_number > bet_number {
                        // WIN
                        // How much to send to winner
                        let win_amount = (self.manager.over_rates[bet_number as usize] as Balance)
                            .checked_mul(bet_amount)
                            .unwrap()
                            .checked_div(10000)
                            .unwrap();
                        if win_amount.checked_sub(bet_amount) > Some(self.env().balance()) {
                            return Err(CoreError::Custom(String::from("O::Not Enough Balance")));
                        }

                        assert!(self.env().transfer(player, win_amount).is_ok());

                        // event
                        self.env().emit_event(WinEvent {
                            player: Some(player),
                            is_over,
                            random_number,
                            bet_number,
                            bet_amount,
                            win_amount,
                        });
                    } else {
                        // LOSE
                        // send to pool
                        let lose_amount = bet_amount
                            .checked_mul(self.manager.revenue_ratio as u128)
                            .unwrap()
                            .checked_div(100)
                            .unwrap();

                        assert!(self
                            .env()
                            .transfer(self.manager.reward_pool, lose_amount)
                            .is_ok());

                        assert!(self
                            .env()
                            .transfer(
                                self.manager.general_pool,
                                bet_amount.checked_sub(lose_amount).unwrap()
                            )
                            .is_ok());

                        // event
                        self.env().emit_event(LoseEvent {
                            player: Some(player),
                            is_over,
                            random_number,
                            bet_number,
                            bet_amount,
                        });
                    }
                } else if is_over == 0 {
                    assert!(
                        (self.manager.min_under_number..=self.manager.max_under_number)
                            .contains(&bet_number)
                    );
                    if random_number < bet_number {
                        // WIN
                        // How much to send to winner
                        let win_amount = (self.manager.under_rates[bet_number as usize] as Balance)
                            .checked_mul(bet_amount)
                            .unwrap()
                            .checked_div(10000)
                            .unwrap();
                        if win_amount.checked_sub(bet_amount) > Some(self.env().balance()) {
                            return Err(CoreError::Custom(String::from("O::Not Enough Balance")));
                        }

                        assert!(self.env().transfer(player, win_amount).is_ok());

                        // event
                        self.env().emit_event(WinEvent {
                            player: Some(player),
                            is_over,
                            random_number,
                            bet_number,
                            bet_amount,
                            win_amount,
                        });
                    } else {
                        // LOSE
                        // send to pool
                        let lose_amount = bet_amount
                            .checked_mul(self.manager.revenue_ratio as u128)
                            .unwrap()
                            .checked_div(100)
                            .unwrap();

                        assert!(self
                            .env()
                            .transfer(self.manager.reward_pool, lose_amount)
                            .is_ok());

                        assert!(self
                            .env()
                            .transfer(
                                self.manager.general_pool,
                                bet_amount.checked_sub(lose_amount).unwrap()
                            )
                            .is_ok());

                        // event
                        self.env().emit_event(LoseEvent {
                            player: Some(player),
                            is_over,
                            random_number,
                            bet_number,
                            bet_amount,
                        });
                    }
                } else {
                    return Err(CoreError::Custom(String::from("O::Invalid Input")));
                }

                // assert!(self.reward_token_to_player(player, bet_amount).is_ok());
                assert!(BetA0CoreTrait::reward_token_to_player(self, player, bet_amount).is_ok());

                // PSP22Ref::mint(&self.manager.psp22,player,bet_amount/(self.manager.token_ratio as u256));
                Ok(())
            } else {
                return Err(CoreError::Custom(String::from("O::Bet Not Exist")));
            }
        }
    }
}
