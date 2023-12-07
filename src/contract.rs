#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use std::env;
use cosmwasm_std::{ to_json_binary, BankQuery, BankMsg, DepsMut, Env, MessageInfo, Response, StdError, WasmMsg, BalanceResponse, Coin, Uint128, Addr, Binary, Deps};
use cw2::set_contract_version;
// use serde::{Deserialize, Serialize};
use nois::{randomness_from_str, pick};
use dotenv::dotenv;



use crate::error::LotteryError;
use crate::helpers::{query_nft_balance, is_round_over, validate_lottery_active};
use crate::msg::{ExecuteMsg, InstantiateMsg, JoinLotteryMsg, UpdateConfigMsg, QueryMsg};
use crate::state::{LotteryState, LOTTERY_STATE, Round};



// version info for migration info
const CONTRACT_NAME: &str = "crates.io:lottery";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, LotteryError> {
     // Initialize your lottery state with the provided parameters
     let lottery_state = LotteryState::new(
        msg.round_duration,
        msg.cooldown_period,
        msg.fee_percentage,
        msg.entry_fee,
        msg.nft_bonus_percentage,
        info.sender.clone().to_string(),
        false,
    );

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    LOTTERY_STATE.save(deps.storage, &lottery_state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.pause_status.to_string())
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, LotteryError> {
    match msg {
        ExecuteMsg::JoinLottery(join_msg) => execute_join_lottery(deps, _env, info, join_msg),
        ExecuteMsg::UpdateConfig(update_msg) => execute_update_config(deps, info, update_msg),
        ExecuteMsg::SelectWinners {} => execute_select_winner(deps, _env, info),
        ExecuteMsg::CreateRound {} => execute_start_new_round(deps, _env, info),
    }
}

fn load_env_var(var_name: &str) -> String {
    dotenv().ok();
    env::var(var_name).expect(&format!("{} not set in .env", var_name))
}

fn get_balance(
    deps: &DepsMut,
    address: String,
) -> Result<u128, StdError> {
    let denom = "inj".to_string();
    let balance_query = BankQuery::Balance { denom, address };
    let balance_response: BalanceResponse = deps.querier.query(&balance_query.into())?;
    let balance_u128 = balance_response.amount.amount.u128();
    Ok(balance_u128)
}

fn select_winner_secure(participants: Vec<String>) -> Result<String, StdError> {
    let randomness_secret = load_env_var("RANDOMNESS_SECRET");
    let randomness: [u8; 32] = randomness_from_str(randomness_secret).map_err(|err| cosmwasm_std::StdError::GenericErr {
        msg: format!("Failed to convert randomness: {}", err),
    })?;

    // Pick a single winner from the list of participants
    let picked = pick(randomness, 1, participants);
    match picked.into_iter().next() {
        Some(winner) => Ok(winner),
        None => Err(StdError::generic_err("No winner selected")),
    }
}


pub fn execute_join_lottery(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: JoinLotteryMsg,
) -> Result<Response, LotteryError> {
    // Ensure the lottery is active
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;

    if !validate_lottery_active(&lottery_state, &env.block.time) {
        return Err(LotteryError::NotActive {});
    }

    // Call query_nft_balance to determine multipliers
    let total_entries: f64 = query_nft_balance(deps.as_ref(), vec![
        Addr::unchecked("inj12lq764spe67c47lg29vcfqe7kdd9fmguxck05l")
        // Addr::unchecked("inj1vtd54v4jm50etkjepgtnd7lykr79yvvah8gdgw"),
        // Addr::unchecked("inj16m9n05n80uylxaafk32qyha38fmwcfpssnpfak"),
        // Addr::unchecked("inj19ly43dgrr2vce8h02a8nw0qujwhrzm9yv8d75c"),
        // Addr::unchecked("inj18u2gdujvg53uy5uh46ev03d77wcp7a4wh4llg0"),
        // Addr::unchecked("inj19zmsda7gdls4pg5fy5cm0hqjr5ue5pgp5uxwcy"),
    ], info.sender.clone())?;
    let mut rounds: Vec<Round> = lottery_state.rounds.clone();

    if let Some(current_round) = rounds.last_mut() {
        let total_entries: f64 = total_entries * msg.number_of_tickets as f64;

        // Update participant field and current entries
        for _ in 0..total_entries.ceil() as u32 {
            current_round.participants.push(info.sender.to_string());
        }

        current_round.current_entries += total_entries as u32;

        // Update the state in storage
        // LOTTERY_STATE.save(deps.storage, lottery_state)?;
        LOTTERY_STATE.save(deps.storage, &lottery_state)?;

         // Deduct the entry fee multiplied by the selected number of tickets from the user's account
         let entry_fee_per_ticket = Uint128::from(lottery_state.entry_fee);
         let total_deduction = entry_fee_per_ticket * Uint128::from(msg.number_of_tickets);

          // Check if the user has enough funds
        if info.funds.len() != 1 || info.funds[0].amount < total_deduction {
            return Err(LotteryError::InsufficientFunds {});
        }


    let entry_fee = Coin {
        denom: "inj".to_string(),
        amount: total_deduction,
    };

    // Clone the msg before using it in WasmMsg::Execute
    let cloned_msg = msg.clone();

    // Return a response
   return Ok(Response::new()
        .add_message(WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_json_binary(&crate::msg::ExecuteMsg::JoinLottery(cloned_msg))?,
            funds: vec![entry_fee],
        })
        .add_attribute("action", "join_lottery")
        .add_attribute("participant", info.sender)
        .add_attribute("round_entered_time", msg.round_entered_time.to_string())
        .add_attribute("round_number", current_round.round_number.to_string())
        .add_attribute("current_entries", current_round.current_entries.to_string()))
    }

     // Handle the case where there are no rounds (possibly an error condition)
     Err(LotteryError::NoRoundAvailable {})

}

pub fn execute_select_winner(deps:DepsMut, env:Env, info: MessageInfo) -> Result<Response, LotteryError> {

    // Get the lottery state
    let mut state = LOTTERY_STATE.load(deps.storage)?;

    if info.sender != state.owner {
        return Err(LotteryError::Unauthorized { sender: info.sender })
    }

    // Ensure that the round is over
   if !is_round_over(&state, &env.block.time) {
       return Err(LotteryError::NotActive {});
   }

   if let Some(current_round) = state.rounds.last_mut() {
    // Call select_winner_secure with the participants and the current round
    let winner = select_winner_secure(current_round.participants.clone())?;
    
    let contract_balance: Result<u128, StdError> = get_balance(&deps, env.contract.address.to_string());
    
    let contract_balance = match contract_balance {
        Ok(balance) => balance,
        Err(err) => return Err(err.into()), // Convert the StdError to an outer error type or return it directly
    };
    
    let winner_prize = (contract_balance * 80) / 100;
    
    // Transfer funds to the winner
    // Construct a message to send the winnings to the winner
    let winner_send_msg = BankMsg::Send {
        to_address: winner.clone(),
        amount: vec![Coin {
            denom: "inj".to_string(),
            amount: winner_prize.into(),
        }],
    };
    
    // Construct a message to send the deployer's share
    let deployer_send_msg = BankMsg::Send {
        to_address: state.owner.clone(),
        amount: vec![Coin {
            denom: "inj".to_string(),
            amount: (contract_balance - winner_prize).into(),
        }],
    };
    
    // Combine the messages
    let messages: Vec<cosmwasm_std::CosmosMsg> =
    vec![winner_send_msg.into(), deployer_send_msg.into()];

    
    // Update the winner for the round in state
     current_round.winner = Some(winner.clone());

    // Update the state in storage
    LOTTERY_STATE.save(deps.storage, &state)?;
    
    // Return the response with the messages to be executed
    Ok(Response::new().add_messages(messages))
}else {
    // Handle the case where there are no rounds (possibly an error condition)
    Err(LotteryError::NoRoundAvailable {})
}

}

pub fn execute_update_config(deps:DepsMut, info: MessageInfo, msg: UpdateConfigMsg) -> Result<Response, LotteryError> {
    // Load lottery state from storage
    let mut lottery_state = LOTTERY_STATE.load(deps.storage)?;

    // Check is caller is owner
    if info.sender != lottery_state.owner {
        return Err(LotteryError::Unauthorized { sender: info.sender })
    }

    if let Some(fee_percentage) = msg.fee_percentage {
        lottery_state.fee_percentage = fee_percentage
    }

    if let Some(nft_bonus_percentage) = msg.nft_bonus_percentage {
        lottery_state.nft_bonus_percentage = nft_bonus_percentage
    }

   if let Some(round_duration) = msg.round_duration {
    lottery_state.round_duration = round_duration;
    }

    if let Some(cooldown_period) = msg.cooldown_period {
        lottery_state.cooldown_period = cooldown_period;
    }

    if let Some(entry_fee) = msg.entry_fee {
        lottery_state.entry_fee = entry_fee;
    }

    if let Some(pause_status) = msg.pause_status {
        lottery_state.pause_status = pause_status;
    }

        // Save the updated state
        LOTTERY_STATE.save(deps.storage, &lottery_state)?;
        // Return a response
        Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn execute_start_new_round(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, LotteryError> {
    // Ensure the caller is the owner (you may want to adjust this based on your contract logic)
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;
    if info.sender != lottery_state.owner {
        return Err(LotteryError::Unauthorized { sender: info.sender });
    }

    // Check if a new round can be started
    if !lottery_state.is_lottery_active(&env.block.time) {
        return Err(LotteryError::NotActive {  });
    }

    // Create a new round and update LotteryState
    let mut lottery_state = lottery_state;
    lottery_state.start_new_round(env.block.time);

    // Save the updated state back to storage
    LOTTERY_STATE.save(deps.storage, &lottery_state)?;

    Ok(Response::new()
        .add_attribute("action", "start_new_round")
        .add_attribute("owner", info.sender)
    )
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, StdError> {
    match msg {

        QueryMsg::LotteryInfo {} => to_json_binary(&query_lottery_state(deps, env)?),
        QueryMsg::CurrentRoundInfo {} => to_json_binary(&query_current_round_info(deps, env)?),
        QueryMsg::RoundInfo(round_info) => to_json_binary(&query_round_info(deps, env, round_info.round_number)?),
    }
}

// // Helper function to query lottery state
fn query_lottery_state(deps: Deps, _env: Env) -> Result<LotteryState, StdError> {
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;
    Ok(lottery_state)
}

// // Helper function to query information about the current round
fn query_current_round_info(deps: Deps, _env: Env) -> Result<Round, StdError> {
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;
    if let Some(current_round) = lottery_state.current_round() {
        Ok(current_round.clone())
    }
    else {
        Err(StdError::GenericErr {
            msg: format!("{}", LotteryError::RoundNotFound {}),
        })
    }
}

fn query_round_info(deps:Deps, _env:Env, round_number:u64) -> Result<Round, StdError> {
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;

    // Find the index of the round
    let round_index = round_number as usize;

    // Check if the round exists
    if let Some(round) = lottery_state.rounds.get(round_index) {
        Ok(round.clone())
    } else {
        Err(StdError::GenericErr {
            msg: format!("{}", LotteryError::RoundNotFound {})
        })
    }
}


