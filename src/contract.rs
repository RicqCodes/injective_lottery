#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_json_binary, BankMsg, DepsMut, Env, MessageInfo, Response, StdError, WasmMsg, Coin, Uint128, Addr, Binary, Deps, ensure_eq};
use cw2::set_contract_version;
use nois::{ProxyExecuteMsg, NoisCallback, ints_in_range};


use crate::state::NOIS_PROXY;
use crate::error::LotteryError;
use crate::helpers::{query_nft_balance, is_round_over, validate_lottery_active, get_balance,};
use crate::msg::{ExecuteMsg, InstantiateMsg, JoinLotteryMsg, QueryMsg};
use crate::state::{LotteryState, LOTTERY_STATE, Round};



// version info for migration info
const CONTRACT_NAME: &str = "crates.io:lottery";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Init is called when the contract is instantiated
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
        msg.win_percentage,
        msg.entry_fee,
        info.sender.clone().to_string(),
        false,
    );

    // NOIS_PROXY.save(deps.storage, &nois_proxy_addr)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    LOTTERY_STATE.save(deps.storage, &lottery_state)?;

       // The nois-proxy abstracts the IBC and nois chain away from this application
       let nois_proxy_addr = deps
        .api
        .addr_validate(&msg.nois_proxy)
        .map_err(|_| LotteryError::InvalidProxyAddress { })?;
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        NOIS_PROXY.save(deps.storage, &nois_proxy_addr)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.pause_status.to_string())
    )
}

// Handle is called when a message is sent to the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, LotteryError> {
    match msg {
        ExecuteMsg::CreateRound {} => execute_start_new_round(deps, _env, info),
        ExecuteMsg::JoinLottery(join_msg) => execute_join_lottery(deps, _env, info, join_msg),
        ExecuteMsg::SelectWinners { job_id } => execute_select_winner(deps, _env, info, job_id),
        ExecuteMsg::UpdateCooldownPeriod(cooldown_period) => execute_update_cooldown_period(deps, _env, info, cooldown_period),
        ExecuteMsg::UpdateEntries(entries) => execute_update_entries(deps, _env, info, entries),
        ExecuteMsg::UpdateEntryFee(entry_fee) => execute_update_entry_fee(deps, _env, info, entry_fee),
        ExecuteMsg::UpdateOwner(owner) => execute_update_owner(deps, _env, info, owner),
        ExecuteMsg::UpdatePauseStatus(pause_status) => execute_update_pause_status(deps, _env, info, pause_status),
        ExecuteMsg::UpdateRoundDuration(round_duration) => execute_update_round_duration(deps, _env, info, round_duration),
        ExecuteMsg::NoisReceive { callback } => execute_receive(deps, _env, info, callback),
    }
}

// Helper function to start new round
pub fn execute_start_new_round(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, LotteryError> {
    // Ensure the caller is the owner
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


// Helper function to join the lottery
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

// Helper function to select the winner
pub fn execute_select_winner(deps: DepsMut, env: Env, info: MessageInfo, job_id: String) -> Result<Response, LotteryError> {
    // Get the lottery state
    let state = LOTTERY_STATE.load(deps.storage)?;

    // Get the nois proxy address
    let nois_proxy = NOIS_PROXY.load(deps.storage)?;

    // Ensure the caller is the owner
    if info.sender != state.owner {
        return Err(LotteryError::Unauthorized { sender: info.sender });
    }

    // Ensure that the round is over
    if !is_round_over(&state.clone(), &env.block.time) {
        return Err(LotteryError::NotActive {});
    }

    let response = Response::new().add_message(WasmMsg::Execute {
        contract_addr: nois_proxy.into(),
        msg: to_json_binary(&ProxyExecuteMsg::GetNextRandomness { job_id })?,
        funds: vec![], // No funds need to be passed
        });

        // Return the response with the messages to be executed
        Ok(response)
}

//The execute_receive function is triggered upon reception of the randomness from the proxy contract
pub fn execute_receive(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    callback: NoisCallback,
) -> Result<Response, LotteryError> {
    //load proxy address from store
    let proxy = NOIS_PROXY.load(deps.storage)?;
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;
    let current_round = lottery_state.current_round().unwrap();

    //callback should only be allowed to be called by the proxy contract
    //otherwise anyone can cut the randomness workflow and cheat the randomness by sending the randomness directly to this contract
    ensure_eq!(info.sender, proxy, LotteryError::UnauthorizedReceive {});

    // In this Dapp we don't need the drand publish time. so we skip it with ..
    let NoisCallback {
        job_id, randomness, ..
    } = callback;

    let randomness: [u8; 32] = randomness
        .to_array()
        .map_err(|_| LotteryError::InvalidRandomness {})?;
    
    // Generate a random index to select a winner from the participants in the current round
    let winner_index = ints_in_range(randomness, 1, 0, current_round.participants.len() as u32 - 1)[0] as usize;

    // Get the winner address
    let winner = current_round.participants[winner_index].clone();

    // Calculate the prize amounts
    let contract_balance = get_balance(&deps, _env.contract.address.to_string())?;
    let winner_prize = (contract_balance * lottery_state.win_percentage as u128) / 100;
    let dev_share = contract_balance - winner_prize;


  // Construct messages to send funds
  let winner_send_msg = BankMsg::Send {
    to_address: winner.clone(),
    amount: vec![Coin {
        denom: "inj".to_string(),
        amount: winner_prize.into(),
    }],
};

let dev_send_msg = BankMsg::Send {
    to_address: lottery_state.owner.clone(),
    amount: vec![Coin {
        denom: "inj".to_string(),
        amount: dev_share.into(),
    }],
};

    // Combine the messages
    let messages: Vec<cosmwasm_std::CosmosMsg> = vec![winner_send_msg.into(), dev_send_msg.into()];

    // Update the winner in the current round
    let mut updated_state = lottery_state.clone();
    updated_state.update_last_winner(Some(winner.clone()))?;
    updated_state.update_job_id(Some(job_id))?;
    LOTTERY_STATE.save(deps.storage, &updated_state)?;

    Ok(Response::new().add_messages(messages))
}


// Helper function to update cooldown period
pub fn execute_update_cooldown_period(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cooldown_period: u64,
) -> Result<Response, LotteryError> {
    // Ensure the caller is the owner
    let mut lottery_state = LOTTERY_STATE.load(deps.storage)?;
    if info.sender != lottery_state.owner {
        return Err(LotteryError::Unauthorized { sender: info.sender });
    }

    // Update the cooldown period
    lottery_state.update_cooldown_period(cooldown_period);

    // Save the updated state back to storage
    LOTTERY_STATE.save(deps.storage, &lottery_state)?;

    Ok(Response::new()
        .add_attribute("action", "update_cooldown_period")
        .add_attribute("owner", info.sender)
        .add_attribute("cooldown_period", cooldown_period.to_string())
    )
}

pub fn execute_update_entries(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    entries: u32,
) -> Result<Response, LotteryError> {
    // Ensure the caller is the owner
    let mut lottery_state = LOTTERY_STATE.load(deps.storage)?;
    if info.sender != lottery_state.owner {
        return Err(LotteryError::Unauthorized { sender: info.sender });
    }

    // Update the entries
    lottery_state.update_entries(entries, info.sender.to_string());

    // Save the updated state back to storage
    LOTTERY_STATE.save(deps.storage, &lottery_state)?;

    Ok(Response::new()
        .add_attribute("action", "update_entries")
        .add_attribute("owner", info.sender)
        .add_attribute("entries", entries.to_string())
    )
}

pub fn execute_update_entry_fee(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    entry_fee: Uint128,
) -> Result<Response, LotteryError> {
    // Ensure the caller is the owner
    let mut lottery_state = LOTTERY_STATE.load(deps.storage)?;
    if info.sender != lottery_state.owner {
        return Err(LotteryError::Unauthorized { sender: info.sender });
    }

    // Update the entry fee
    lottery_state.update_entry_fee(entry_fee);

    // Save the updated state back to storage
    LOTTERY_STATE.save(deps.storage, &lottery_state)?;

    Ok(Response::new()
        .add_attribute("action", "update_entry_fee")
        .add_attribute("owner", info.sender)
        .add_attribute("entry_fee", entry_fee.to_string())
    )
}

pub fn execute_update_owner(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: String,
) -> Result<Response, LotteryError> {
    // Ensure the caller is the owner
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;
    if info.sender != lottery_state.owner {
        return Err(LotteryError::Unauthorized { sender: info.sender });
    }

    // Update the owner
    lottery_state.clone().update_owner(owner.to_string());

    // Save the updated state back to storage
    LOTTERY_STATE.save(deps.storage, &lottery_state)?;

    Ok(Response::new()
        .add_attribute("action", "update_owner")
        .add_attribute("owner", info.sender)
        .add_attribute("new_owner", owner)
    )
}

pub fn execute_update_pause_status(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    pause_status: bool,
) -> Result<Response, LotteryError> {
    // Ensure the caller is the owner
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;
    if info.sender != lottery_state.owner {
        return Err(LotteryError::Unauthorized { sender: info.sender });
    }

    // Update the pause status
    lottery_state.clone().update_pause_status(pause_status);

    // Save the updated state back to storage
    LOTTERY_STATE.save(deps.storage, &lottery_state)?;

    Ok(Response::new()
        .add_attribute("action", "update_pause_status")
        .add_attribute("owner", info.sender)
        .add_attribute("pause_status", pause_status.to_string())
    )
}

pub fn execute_update_round_duration(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    round_duration: u64,
) -> Result<Response, LotteryError> {
    // Ensure the caller is the owner
    let mut lottery_state = LOTTERY_STATE.load(deps.storage)?;
    if info.sender != lottery_state.owner {
        return Err(LotteryError::Unauthorized { sender: info.sender });
    }

    // Update the round duration
    lottery_state.update_round_duration(round_duration);

    // Save the updated state back to storage
    LOTTERY_STATE.save(deps.storage, &lottery_state)?;

    Ok(Response::new()
        .add_attribute("action", "update_round_duration")
        .add_attribute("owner", info.sender)
        .add_attribute("round_duration", round_duration.to_string())
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

// Helper function to query lottery state
fn query_lottery_state(deps: Deps, _env: Env) -> Result<LotteryState, StdError> {
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;
    Ok(lottery_state)
}

// Helper function to query information about the current round
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


