use cosmwasm_std::{ Uint128, Timestamp};
use nois::NoisCallback;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub round_duration: u64,
    pub cooldown_period: u64,
    pub win_percentage: u32,
    pub entry_fee: Uint128,
    pub pause_status: bool,
    pub nois_proxy: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    JoinLottery(JoinLotteryMsg),
    UpdateEntries(u32),
    UpdateEntryFee(Uint128),
    UpdateCooldownPeriod(u64),
    UpdateRoundDuration(u64),
    UpdateOwner(String),
    UpdatePauseStatus(bool),
    SelectWinners {
        job_id: String,
    },
    NoisReceive { callback: NoisCallback },
    CreateRound,
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub enum QueryMsg {
    LotteryInfo { },
    CurrentRoundInfo { },
    RoundInfo (RoundInfoQuery),
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct JoinLotteryMsg {
    pub sender: String,
    pub number_of_tickets:u32,
    pub round_entered_time:Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoundInfoQuery {
    pub round_number: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParticipantInfoQuery {
    pub participant_address: String,
}


