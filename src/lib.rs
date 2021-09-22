mod state;
mod instructions;
mod entrypoint;
mod error;
mod processor;

#[cfg(test)]
mod tests {
    use crate::instructions::TokenInstruction;
    use crate::processor::process;
    use crate::state::{Account, Mint};

    use solana_program::{
        account_info::AccountInfo, instruction::AccountMeta, program_pack::Pack, pubkey::Pubkey, sysvar,
    };

    use solana_sdk::account::{
        create_account_for_test, create_is_signer_account_infos, Account as SolanaAccount,
    };


    #[test]
    fn test_initialize_mint_success() {
        let program_id = Pubkey::new_unique();

        let (
            mint_key,
            mut mint,
            mint_meta
        ) = _generate_account(4242424242, Mint::get_packed_len(), false);

        let (
            account_key,
            mut account,
            account_meta
        ) = _generate_account(4242424242, Account::get_packed_len(), false);

        let (
            owner_key,
            mut owner,
            owner_meta
        ) = _generate_account(4242424242, Account::get_packed_len(), false);

        let mut rent_sysvar = _rent_sysvar();

        let accounts_metas = vec![
            mint_meta,
            account_meta,
            owner_meta,
            AccountMeta::new(sysvar::rent::id(), false),
        ];

        let accounts = vec![&mut mint, &mut account, &mut owner, &mut rent_sysvar];

        _run_test(accounts_metas, accounts, _init_mint);
    }

    #[test]
    fn test_initialize_account_success() {
        let (
            mint_key,
            mut mint,
            mint_meta
        ) = _generate_account(4242424242, Mint::get_packed_len(), false);
        Mint::pack_into_slice(&Mint { is_initialized: true }, mint.data.as_mut());

        let (
            account_key,
            mut account,
            account_meta
        ) = _generate_account(4242424242, Account::get_packed_len(), false);

        let (owner_key,
            mut owner,
            owner_meta
        ) = _generate_account(4242424242, Account::get_packed_len(), false);

        let mut rent_sysvar = _rent_sysvar();

        let accounts_metas = vec![
            account_meta,
            mint_meta,
            owner_meta,
            AccountMeta::new(sysvar::rent::id(), false),
        ];

        let accounts = vec![&mut account, &mut mint, &mut owner, &mut rent_sysvar];

        _run_test(accounts_metas, accounts, _init_account);
    }


    fn _run_test<F>(accounts_metas: Vec<AccountMeta>, accounts: Vec<&mut SolanaAccount>, process: F) where F: Fn(&Pubkey, &mut [AccountInfo]) {
        let mut meta = accounts_metas
            .iter()
            .zip(accounts)
            .map(|(account_meta, account)| (&account_meta.pubkey, account_meta.is_signer, account))
            .collect::<Vec<_>>();

        let mut account_infos = create_is_signer_account_infos(&mut meta);

        let program_id = Pubkey::new_unique();

        process(&program_id, account_infos.as_mut_slice());
    }

    fn _init_mint(program_id: &Pubkey, accounts: &mut [AccountInfo]) {
        assert_eq!(process(program_id, accounts, TokenInstruction::InitializeMint.pack().as_slice()), Ok(()));

        let account_index = 1;
        let account = &accounts[account_index];
        let account_data = Account::unpack_from_slice(&account.data.as_ref().borrow());

        let mint_index = 0;
        let mint = &accounts[mint_index];

        let owner_index = 2;
        let owner = &accounts[owner_index];

        assert!(
            if let Ok(data) = account_data {
                assert!(data.is_initialized);
                assert_eq!(data.amount, 1);
                assert_eq!(data.mint, *mint.key);
                assert_eq!(data.owner, *owner.key);
                true
            } else {
                false
            }
        );
    }

    fn _init_account(program_id: &Pubkey, accounts: &mut [AccountInfo]) {
        assert_eq!(process(program_id, accounts, TokenInstruction::InitializeAccount.pack().as_slice()), Ok(()));

        let account_index = 0;
        let account = &mut accounts[account_index];
        let account_data = Account::unpack_from_slice(&account.data.as_ref().borrow());

        let mint_index = 1;
        let mint = &accounts[mint_index];

        let owner_index = 2;
        let owner = &accounts[owner_index];

        assert!(
            if let Ok(data) = account_data {
                assert!(data.is_initialized);
                assert_eq!(data.amount, 0);
                assert_eq!(data.mint, *mint.key);
                assert_eq!(data.owner, *owner.key);
                true
            } else {
                false
            }
        );
    }

    fn _generate_account(lamports: u64, space: usize, signer: bool) -> (Pubkey, SolanaAccount, AccountMeta) {
        let acc_key = Pubkey::new_unique();
        let acc = SolanaAccount::new(lamports, space, &acc_key);
        let acc_meta = AccountMeta::new(acc_key, signer);
        return (acc_key, acc, acc_meta);
    }

    fn _rent_sysvar() -> SolanaAccount {
        create_account_for_test(&sysvar::rent::Rent::default())
    }
}
