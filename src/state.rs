use cosmwasm_std::{Addr, StdError, StdResult, Timestamp, Uint128};
use schemars::JsonSchema;
// use cosmwasm_storage::{singleton, singleton_read};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, JsonSchema, Serialize, Deserialize)]
pub struct Round {
    pub round_number: u64,
    pub round_start_time: Timestamp,
    pub current_entries: u32,
    pub participants: Vec<String>,
    pub winner: Option<String>,
    pub job_id: Option<String>,
}

impl Round {
    pub fn new(round_number: u64, round_start_time: Timestamp) -> Self {
        Round {
            round_number,
            round_start_time,
            current_entries: 0,
            participants: Vec::new(),
            winner: None,
            job_id: None,
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq, JsonSchema, Serialize, Deserialize)]
pub struct LotteryState {
    pub rounds: Vec<Round>,
    pub round_duration: u64,
    pub cooldown_period: u64,
    pub win_percentage: u32,
    pub entry_fee: Uint128,
    pub nft_bonus_percentage: u32,
    pub owner: String,
    pub pause_status: bool,
}

impl LotteryState {
    /// Creates a new instance of `LotteryState`.
    ///
    /// # Arguments
    ///
    /// * `round_duration` - The duration of each round in seconds.
    /// * `cooldown_period` - The cooldown period in seconds.
    /// * `win_percentage` - The fee percentage for each entry.
    /// * `entry_fee` - The entry fee for each participant.
    /// * `owner` - The owner of the lottery.
    /// * `pause_status` - The pause status of the lottery.
    ///
    /// # Returns
    ///
    /// A new instance of `LotteryState`.
    pub fn new(
        round_duration: u64,
        cooldown_period: u64,
        win_percentage: u32,
        entry_fee: Uint128,
        owner: String,
        pause_status: bool,
    ) -> Self {
        LotteryState {
            round_duration,
            cooldown_period,
            win_percentage,
            entry_fee,
            owner,
            pause_status,
            ..Default::default()
        }
    }

    /// Returns a reference to the current round of the lottery.
    ///
    /// # Returns
    ///
    /// A reference to the current round of the lottery, or `None` if there are no rounds.
    pub fn current_round(&self) -> Option<&Round> {
        self.rounds.last()
    }

    /// Updates the entries and participant for the current round of the lottery.
    ///
    /// # Arguments
    ///
    /// * `entries` - The number of entries to be added.
    /// * `participant` - The participant to be added.
    pub fn update_entries(&mut self, entries: u32, participant: String) {
        if let Some(current_round) = self.rounds.last_mut() {
            current_round.current_entries += entries;
            current_round.participants.push(participant.to_string());
        }
    }

    /// Updates the fee percentage for each entry in the lottery.
    ///
    /// # Arguments
    ///
    /// * `fee_percentage` - The new fee percentage.
    pub fn update_fee_percentage(&mut self, win_percentage: u32) {
        self.win_percentage = win_percentage;
    }

    /// Updates the entry fee for each participant in the lottery.
    ///
    /// # Arguments
    ///
    /// * `entry_fee` - The new entry fee.
    pub fn update_entry_fee(&mut self, entry_fee: Uint128) {
        self.entry_fee = entry_fee;
    }

    /// Updates the cooldown period in seconds for the lottery.
    ///
    /// # Arguments
    ///
    /// * `cooldown_period` - The new cooldown period in seconds.
    pub fn update_cooldown_period(&mut self, cooldown_period: u64) {
        self.cooldown_period = cooldown_period;
    }

    /// Updates the round duration in seconds for the lottery.
    ///
    /// # Arguments
    ///
    /// * `round_duration` - The new round duration in seconds.
    pub fn update_round_duration(&mut self, round_duration: u64) {
        self.round_duration = round_duration;
    }

    /// Updates the last winner of the lottery.
    ///
    /// # Arguments
    ///
    /// * `winner` - The new winner.
    pub fn update_last_winner(&mut self, winner: Option<String>) -> StdResult<()> {
        if let Some(current_round) = self.rounds.last_mut() {
            current_round.winner = winner;
            Ok(())
        } else {
            Err(StdError::generic_err("No rounds available"))
        }
    }

    /// Updates the job ID for the lottery.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The new job ID.
    pub fn update_job_id(&mut self, job_id: Option<String>) -> StdResult<()> {
        if let Some(current_round) = self.rounds.last_mut() {
            current_round.job_id = job_id;
            Ok(())
        } else {
            Err(StdError::generic_err("No rounds available"))
        }
    }

    /// Updates the owner of the lottery contract.
    ///
    /// # Arguments
    ///
    /// * `owner` - The new owner.
    pub fn update_owner(mut self, owner: String) -> Self {
        self.owner = owner;
        self
    }

    /// Updates the pause status of the lottery contract.
    ///
    /// # Arguments
    ///
    /// * `pause_status` - The new pause status.
    pub fn update_pause_status(mut self, pause_status: bool) -> Self {
        self.pause_status = pause_status;
        self
    }

    /// Checks if the cooldown period is over.
    ///
    /// # Arguments
    ///
    /// * `current_time` - The current time.
    ///
    /// # Returns
    ///
    /// `true` if the cooldown period is over, `false` otherwise.
    pub fn is_cooldown_over(&self, current_time: &Timestamp) -> bool {
        if let Some(last_round) = self.rounds.last() {
            let cooldown_end_time = last_round
                .round_start_time
                .plus_seconds(self.cooldown_period);
            return current_time >= &cooldown_end_time;
        }

        // If there are no rounds, consider the cooldown period as over
        true
    }

    /// Starts a new round of the lottery.
    ///
    /// # Arguments
    ///
    /// * `round_start_time` - The start time of the new round.
    pub fn start_new_round(&mut self, round_start_time: Timestamp) {
        let round_number = self.rounds.len() as u64 + 1;
        let round = Round::new(round_number, round_start_time);
        self.rounds.push(round);
    }

    /// Checks if the lottery is active.
    ///
    /// # Arguments
    ///
    /// * `current_time` - The current time.
    pub fn is_lottery_active(&self, current_time: &Timestamp) -> Result<bool, String> {
        if let Some(current_round) = self.rounds.last() {
            let round_end_time = current_round.round_start_time.plus_seconds(self.round_duration);
            let within_round_duration = current_time <= &round_end_time;
            let not_paused = !self.pause_status;
    
            if within_round_duration && not_paused {
                Ok(true)
            } else {
                // Check for cooldown period
                let cooldown_end_time = round_end_time.plus_seconds(self.cooldown_period);
                if current_time <= &cooldown_end_time {
                    Err("Cool-down period is active.".to_string())
                } else {
                    Err("Round is still active.".to_string())
                }
            }
        } else {
            Ok(false)
        }
    }
    

}

pub const LOTTERY_STATE: Item<LotteryState> = Item::new("lottery_state");
pub const NOIS_PROXY: Item<Addr> = Item::new("nois_proxy");
