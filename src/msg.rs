use cosmwasm_std::{ Uint128, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub round_duration: u64,
    pub cooldown_period: u64,
    pub fee_percentage: u32,
    pub entry_fee: Uint128,
    pub nft_bonus_percentage: u32,
    pub pause_status: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    JoinLottery(JoinLotteryMsg),
    UpdateConfig(UpdateConfigMsg),
    SelectWinners {},
    CreateRound
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
pub struct UpdateConfigMsg {
    pub fee_percentage: Option<u32>,
    pub nft_bonus_percentage: Option<u32>,
    pub round_duration: Option<u64>,
    pub cooldown_period: Option<u64>,
    pub entry_fee: Option<Uint128>,
    pub pause_status: Option<bool>,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoundInfoQuery {
    pub round_number: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParticipantInfoQuery {
    pub participant_address: String,
}


