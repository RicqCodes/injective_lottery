use cosmwasm_std::{StdError, Addr};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LotteryError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("No rounds available")]
    NoRoundAvailable {},

    #[error("Lottery is not active")]
    NotActive {},

    #[error("Invalid msg type")]
    InvalidMsgType {},

    #[error("Round not found")]
    RoundNotFound {},

    #[error("Nft query failed")]
    NftQueryFaied {},

    #[error("Cannot retrieve balance")]
    BalanceFailed {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    // Unused error case. Zero is now treated like every other value.
    #[deprecated(note = "Unused. All zero amount checks have been removed")]
    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Invalid expiration value")]
    InvalidExpiration {},

    #[error("Proxy address is not valid")]
    InvalidProxyAddress {},

    #[error("Unauthorized Received")]
    UnauthorizedReceive {},

    #[error("Invalid Randomness")]
    InvalidRandomness {},

    #[error("Duplicate initial balance addresses")]
    DuplicateInitialBalanceAddresses {},
}

