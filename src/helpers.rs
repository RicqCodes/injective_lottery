use crate::state::LotteryState;
use cosmwasm_std::{
    to_json_binary, Addr, BalanceResponse, BankQuery, Deps, DepsMut, QueryRequest, StdError,
    StdResult, Timestamp, WasmQuery,
};
use cw721::Cw721QueryMsg;

// Helper function to calculate total entries based on multipliers for different NFT counts
fn calculate_entries(nft_count: u32) -> f64 {
    match nft_count {
        1 => 1.5,
        2 => 2.0,
        3 => 2.5,
        _ => 1.0,
    }
}

// Helper function to query the balance of NFTs from multiple CW721 contracts
pub fn query_nft_balance(deps: Deps, cw721_contracts: Vec<Addr>, owner: Addr) -> StdResult<f64> {
    let mut total_nft_count = 0;
    let mut i = 0;

    while i < cw721_contracts.len() {
        if total_nft_count == 3 {
            break;
        }

        let cw721_contract = &cw721_contracts[i];

        // Use the actual query message for your CW721 contract
        let query_msg = Cw721QueryMsg::Tokens {
            owner: owner.clone().into(),
            start_after: None,
            limit: Some(3),
        };

        let tokens: Vec<String> = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: cw721_contract.to_string(),
            msg: to_json_binary(&query_msg)?,
        }))?;

        // If the user has 3 or more tokens, treat it as having 3, and discard previous calculations
        if tokens.len() as u32 >= 3 {
            total_nft_count = 3;
            break; // Stop the loop if token count is >=3
        } else if tokens.len() as u32 == 2 {
            total_nft_count += 1;
        } else {
            // Otherwise, update entries based on the actual token count
            total_nft_count += tokens.len() as u32;
        }

        i += 1;
    }

    let total_entries = calculate_entries(total_nft_count);

    Ok(total_entries)
}

// Helper function to check if the round is over
pub fn is_round_over(state: &LotteryState, current_time: &Timestamp) -> bool {
    if let Some(current_round) = state.rounds.last() {
        // Calculate the end time of the current round by adding round duration to round start time
        let round_end_time = current_round
            .round_start_time
            .plus_seconds(state.round_duration);

        // Check if the current time is greater than or equal to the end time of the round
        return current_time >= &round_end_time;
    }

    // Handle the case where there are no rounds (possibly an error condition)
    false
}

// Helper function to check if the cooldown period is over
pub fn validate_lottery_active(state: &LotteryState, current_time: &Timestamp) -> bool {
    state.is_lottery_active(current_time).unwrap_or(false)
}

// Helper function to get the balance of the contract
pub fn get_balance(deps: &DepsMut, address: String) -> Result<u128, StdError> {
    let denom = "inj".to_string();
    let balance_query = BankQuery::Balance { denom, address };
    let balance_response: BalanceResponse = deps.querier.query(&balance_query.into())?;
    let balance_u128 = balance_response.amount.amount.u128();
    Ok(balance_u128)
}
