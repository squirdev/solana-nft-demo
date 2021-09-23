use thiserror::Error;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use num_derive::FromPrimitive;

// Errors that may be returned by the Token program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum TokenError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Already in use")]
    AlreadyInUse,
    #[error("Owner does not match")]
    OwnerMismatch,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Account not associated with this Mint")]
    MintMismatch,
    #[error("State does not initialized")]
    NotInitialized,
}

impl From<TokenError> for ProgramError {
    fn from(e: TokenError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for TokenError {
    fn type_of() -> &'static str {
        "TokenError"
    }
}