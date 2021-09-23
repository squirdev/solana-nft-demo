use solana_program::program_error::ProgramError;

use crate::{error::TokenError};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenInstruction {
    /// Initializes a new mint and optionally deposits minted
    /// token in an account.
    ///
    /// The `InitializeMint` instruction requires no signers and MUST be
    /// included within the same Transaction as the system program's
    /// `CreateAccount` instruction that creates the account being initialized.
    /// Otherwise another party can acquire ownership of the uninitialized
    /// account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   1. `[writable]` The mint to initialize.
    ///   1. `[writable]` The account to initialize.
    ///   2. `[]` The new account's owner.
    ///
    InitializeMint,

    /// Initializes a new account to hold token.
    ///
    /// The `InitializeAccount` instruction requires no signers and MUST be
    /// included within the same Transaction as the system program's
    /// `CreateAccount` instruction that creates the account being initialized.
    /// Otherwise another party can acquire ownership of the uninitialized
    /// account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]`  The account to initialize.
    ///   1. `[]` The mint this account will be associated with.
    ///   2. `[]` The new account's owner.
    InitializeAccount,

    /// Transfers token from one account to another either directly or via a
    /// delegate.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single owner
    ///   0. `[writable]` The source account.
    ///   1. `[writable]` The destination account.
    ///   2. `[signer]` The source account's owner.
    Transfer,
}

impl TokenInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Ok(match input[0] {
            0 => TokenInstruction::InitializeMint,
            1 => TokenInstruction::InitializeAccount,
            2 => TokenInstruction::Transfer,
            _ => return Err(TokenError::InvalidInstruction.into()),
        })
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            TokenInstruction::InitializeMint => buf.push(0),
            TokenInstruction::InitializeAccount => buf.push(1),
            TokenInstruction::Transfer => buf.push(2),
        }
        buf
    }
}