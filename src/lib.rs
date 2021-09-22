use solana_program::pubkey::Pubkey;

mod state;
mod instructions;
mod entrypoint;
mod error;
mod processor;

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{
        account_info::IntoAccountInfo, clock::Epoch, instruction::Instruction, program_error,
        sysvar,
    };

    use solana_sdk::account::{
        create_account_for_test, create_is_signer_account_infos, Account as SolanaAccount,
    };

    use crate::instructions::TokenInstruction;
    use solana_program::account_info::AccountInfo;
    use solana_program::program_pack::Pack;
    use solana_program::instruction::AccountMeta;
    use crate::state::Account;

    #[test]
    fn test_initialize_mint_success() {
        let program_id = Pubkey::new_unique();

        let mint_key = Pubkey::new_unique();
        let mut mint = SolanaAccount::new(4242424242, state::Mint::get_packed_len(), &mint_key);

        let account_key = Pubkey::new_unique();
        let mut account = SolanaAccount::new(4242424242, state::Account::get_packed_len(), &account_key);

        let owner_key = Pubkey::new_unique();
        let mut owner = SolanaAccount::new(4242424242, state::Account::get_packed_len(), &owner_key);

        let mut rent_sysvar = rent_sysvar();

        let mut accounts_metas = vec![
            AccountMeta::new(mint_key, false),
            AccountMeta::new(account_key, false),
            AccountMeta::new(owner_key, false),
            AccountMeta::new(sysvar::rent::id(), false),
        ];

        let mut accounts = vec![&mut mint, &mut account, &mut owner, &mut rent_sysvar];

        let mut meta = accounts_metas
            .iter()
            .zip(accounts)
            .map(|(account_meta, account)| (&account_meta.pubkey, account_meta.is_signer, account))
            .collect::<Vec<_>>();

        let account_infos = create_is_signer_account_infos(&mut meta);

        assert_eq!(processor::process(&program_id, account_infos.as_ref(), TokenInstruction::InitializeMint.pack().as_slice()), Ok(()));

        let account_data = Account::unpack_from_slice(account.data.as_ref());
        assert!(if let Ok(data) = account_data {
            assert_eq!(data.amount, 1);
            true
        } else {
            false
        });
    }

    fn rent_sysvar() -> SolanaAccount {
        create_account_for_test(&sysvar::rent::Rent::default())
    }
}
