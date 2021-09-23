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
        ) = _generate_account_with_owner(4242424242, Mint::get_packed_len(), false, &program_id);

        let (
            account_key,
            mut account,
            account_meta
        ) = _generate_account_with_owner(4242424242, Account::get_packed_len(), false, &program_id);

        let (
            owner_key,
            mut owner,
            owner_meta
        ) = _generate_account_with_owner(4242424242, Account::get_packed_len(), false, &program_id);


        let accounts_metas = vec![
            mint_meta,
            account_meta,
            owner_meta,
        ];

        let accounts = vec![&mut mint, &mut account, &mut owner];

        _run_test(accounts_metas, accounts, _init_mint);
    }

    #[test]
    fn test_initialize_account_success() {
        let program_id = Pubkey::new_unique();

        let (
            mint_key,
            mut mint,
            mint_meta
        ) = _generate_account_with_owner(4242424242, Mint::get_packed_len(), false, &program_id);
        Mint::pack_into_slice(&Mint { is_initialized: true }, mint.data.as_mut());

        let (
            account_key,
            mut account,
            account_meta
        ) = _generate_account_with_owner(4242424242, Account::get_packed_len(), false, &program_id);

        let (owner_key,
            mut owner,
            owner_meta
        ) = _generate_account_with_owner(4242424242, Account::get_packed_len(), false, &program_id);

        let accounts_metas = vec![
            account_meta,
            mint_meta,
            owner_meta,
        ];

        let accounts = vec![&mut account, &mut mint, &mut owner];

        _run_test(accounts_metas, accounts, _init_account);
    }

    #[test]
    fn test_transfer_success() {
        let program_id = Pubkey::new_unique();

        let mint_key = Pubkey::new_unique();

        let (
            owner_key,
            mut owner,
            owner_meta
        ) = _generate_account_with_owner(4242424242, Account::get_packed_len(), true, &program_id);

        let (
            source_key,
            mut source,
            source_meta
        ) = _generate_account_with_owner(4242424242, Account::get_packed_len(), false, &program_id);

        Account::pack_into_slice(&Account {
            mint: mint_key,
            owner: owner_key,
            amount: 1,
            is_initialized: true,
        }, source.data.as_mut());

        let (
            dest_key,
            mut dest,
            dest_meta
        ) = _generate_account_with_owner(4242424242, Account::get_packed_len(), false, &program_id);

        Account::pack_into_slice(&Account {
            mint: mint_key,
            owner: owner_key,
            amount: 0,
            is_initialized: true,
        }, dest.data.as_mut());


        let accounts_metas = vec![
            source_meta,
            dest_meta,
            owner_meta,
        ];

        let accounts = vec![&mut source, &mut dest, &mut owner];

        _run_test(accounts_metas, accounts, _transfer);
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

    fn _transfer(program_id: &Pubkey, accounts: &mut [AccountInfo]) {
        assert_eq!(process(program_id, accounts, TokenInstruction::Transfer.pack().as_slice()), Ok(()));

        let source_index = 0;
        let source = &mut accounts[source_index];
        let source_data = Account::unpack_from_slice(&source.data.as_ref().borrow());
        assert!(source_data.is_ok());

        let dest_index = 1;
        let dest = &mut accounts[dest_index];
        let dest_data = Account::unpack_from_slice(&dest.data.as_ref().borrow());
        assert!(dest_data.is_ok());


        let source_data = source_data.unwrap();
        let dest_data = dest_data.unwrap();

        assert_eq!(source_data.amount, 0);
        assert_eq!(dest_data.amount, 1);
    }

    fn _generate_account(lamports: u64, space: usize, signer: bool) -> (Pubkey, SolanaAccount, AccountMeta) {
        let owner = Pubkey::new_unique();
        _generate_account_with_owner(lamports, space, signer, &owner)
    }

    fn _generate_account_with_owner(lamports: u64, space: usize, signer: bool, owner: &Pubkey) -> (Pubkey, SolanaAccount, AccountMeta) {
        let acc_key = Pubkey::new_unique();
        let acc = SolanaAccount::new(lamports, space, owner);
        let acc_meta = AccountMeta::new(acc_key, signer);
        return (acc_key, acc, acc_meta);
    }

    fn _rent_sysvar() -> SolanaAccount {
        create_account_for_test(&sysvar::rent::Rent::default())
    }
}
