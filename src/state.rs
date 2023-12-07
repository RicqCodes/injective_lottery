use cosmwasm_std::{StdResult, Timestamp, StdError, Uint128};
use schemars::JsonSchema;
// use cosmwasm_storage::{singleton, singleton_read};
use serde::{Serialize, Deserialize};
use cw_storage_plus::Item;

#[derive(Debug, Default, Clone, PartialEq, JsonSchema, Serialize, Deserialize)]
pub struct Round {
    pub round_number: u64,
    pub round_start_time: Timestamp,
    pub current_entries: u32,
    pub participants: Vec<String>,
    pub winner: Option<String>,
}

impl Round {
    pub fn new(round_number: u64, round_start_time: Timestamp) -> Self {
        Round {
            round_number,
            round_start_time,
            current_entries: 0,
            participants: Vec::new(),
            winner: None,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, JsonSchema, Serialize, Deserialize)]
pub struct LotteryState {
    pub rounds: Vec<Round>,
    pub round_duration: u64,
    pub cooldown_period: u64,
    pub fee_percentage: u32,
    pub entry_fee: Uint128,
    pub nft_bonus_percentage: u32,
    pub owner: String,
    pub pause_status: bool,
}

impl LotteryState {
    pub fn new(
        round_duration: u64,
        cooldown_period: u64,
        fee_percentage: u32,
        entry_fee: Uint128,
        nft_bonus_percentage: u32,
        owner: String,
        pause_status: bool,
    ) -> Self {
        LotteryState {
            round_duration,
            cooldown_period,
            fee_percentage,
            entry_fee,
            nft_bonus_percentage,
            owner,
            pause_status,
            ..Default::default()
        }
    }

    pub fn current_round(&self) -> Option<&Round> {
        self.rounds.last()
    }

    pub fn update_entries(&mut self, entries: u32, participant: String) {
        if let Some(current_round) = self.rounds.last_mut() {
            current_round.current_entries += entries;
            current_round.participants.push(participant.to_string());
        }
    }

    pub fn update_fee_percentage(&mut self, fee_percentage: u32) {
        self.fee_percentage = fee_percentage;
    }

    pub fn update_last_winner(&mut self, winner: Option<String>) -> StdResult<()> {
        if let Some(current_round) = self.rounds.last_mut() {
            current_round.winner = winner;
            Ok(())
        } else {
            Err(StdError::generic_err("No rounds available").into())
        }
    }

    pub fn update_nft_bonus_percentage(&mut self, nft_bonus_percentage: u32) {
        self.nft_bonus_percentage = nft_bonus_percentage;
    }

    pub fn with_owner(mut self, owner: String) -> Self {
        self.owner = owner;
        self
    }

    pub fn with_pause_status(mut self, pause_status: bool) -> Self {
        self.pause_status = pause_status;
        self
    }

    pub fn is_cooldown_over(&self, current_time: &Timestamp) -> bool {
        if let Some(last_round) = self.rounds.last() {
            let cooldown_end_time = last_round.round_start_time.plus_seconds(self.cooldown_period);
            return current_time >= &cooldown_end_time;
        }

        // If there are no rounds, consider the cooldown period as over
        true
    }

    pub fn start_new_round(&mut self, round_start_time: Timestamp) {
        let round_number = self.rounds.len() as u64 + 1;
        let round = Round::new(round_number, round_start_time);
        self.rounds.push(round);
    }

    pub fn is_lottery_active(&self, current_time: &Timestamp) -> bool {
        if let Some(current_round) = self.rounds.last() {
            let round_end_time = current_round.round_start_time.plus_seconds(self.round_duration);
            let within_round_duration = current_time <= &round_end_time;
            let not_paused = !self.pause_status;

            return within_round_duration && not_paused;
        }
        false
    }
}

pub const LOTTERY_STATE: Item<LotteryState> = Item::new("lottery_state");


