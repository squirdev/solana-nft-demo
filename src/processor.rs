use crate::{
    error::TokenError,
    instructions::TokenInstruction,
    state::Mint,
};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use crate::state::Account;


pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = TokenInstruction::unpack(input)?;
    return match instruction {
        TokenInstruction::InitializeMint => process_initialize_mint(accounts),
        TokenInstruction::InitializeAccount => process_initialize_account(accounts),

        TokenInstruction::Transfer => {
            Ok(())
        }
    };
}

pub fn process_initialize_mint(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let mint_info = next_account_info(account_info_iter)?;
    let account_info = next_account_info(account_info_iter)?;

    let owner_info = next_account_info(account_info_iter)?;
    let rent = Rent::from_account_info(next_account_info(account_info_iter)?)?;

    _init_mint(mint_info, rent)?;
    _init_account(1, account_info, &mint_info, &owner_info, rent)?;

    Ok(())
}

fn _init_mint(mint_info: &AccountInfo, rent: Rent) -> ProgramResult {
    let mint_data_len = mint_info.data_len();
    if !rent.is_exempt(mint_info.lamports(), mint_data_len) {
        return Err(TokenError::NotRentExempt.into());
    }

    let mut mint = _get_mint(mint_info)?;

    mint.is_initialized = true;
    Mint::pack(mint, &mut mint_info.data.as_ref().borrow_mut())
}

fn _init_account(amount: u8, account_info: &AccountInfo, mint_info: &AccountInfo, owner_info: &AccountInfo, rent: Rent) -> ProgramResult {
    let account_data_len = account_info.data_len();
    if !rent.is_exempt(account_info.lamports(), account_data_len) {
        return Err(TokenError::NotRentExempt.into());
    }

    let mut account = _get_account(account_info)?;

    account.mint = *mint_info.key;
    account.owner = *owner_info.key;
    account.amount = amount;
    account.is_initialized = true;
    Account::pack(account, &mut account_info.data.as_ref().borrow_mut())
}

fn _get_account(account_info: &AccountInfo) -> Result<Account, ProgramError> {
    let account = Account::unpack_unchecked(&account_info.data.as_ref().borrow())?;
    if account.is_initialized {
        return Err(TokenError::AlreadyInUse.into());
    }

    Ok(account)
}

fn _get_mint(mint_info: &AccountInfo) -> Result<Mint, ProgramError> {
    let mint = Mint::unpack_unchecked(&mint_info.data.as_ref().borrow())?;
    if mint.is_initialized {
        return Err(TokenError::AlreadyInUse.into());
    }

    Ok(mint)
}

pub fn process_initialize_account(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let account_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;

    let owner_info = next_account_info(account_info_iter)?;
    let rent = Rent::from_account_info(next_account_info(account_info_iter)?)?;

    _init_account(0, account_info, &mint_info, &owner_info, rent)
}

pub fn process_transfer(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let source_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;
    let signer_info = next_account_info(account_info_iter)?;

    _validate_owner(signer_info.key, signer_info)?;

    let mut source = _get_account(source_info)?;
    let mut destination = _get_account(destination_info)?;

    if source.amount == 0 {
        return Err(TokenError::InsufficientFunds.into());
    }

    if source.mint != destination.mint {
        return Err(TokenError::MintMismatch.into());
    }

    source.amount = 0;
    destination.amount = 1;

    Account::pack(source, &mut source_info.data.as_ref().borrow_mut())?;
    Account::pack(destination, &mut destination_info.data.as_ref().borrow_mut())?;
    Ok(())
}

fn _validate_owner(expected_owner: &Pubkey, owner_info: &AccountInfo) -> ProgramResult {
    if expected_owner != owner_info.key {
        return Err(TokenError::OwnerMismatch.into());
    }

    if !owner_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    Ok(())
}