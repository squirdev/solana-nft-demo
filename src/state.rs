use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mint {
    pub is_initialized: bool,
}

impl Sealed for Mint {}

impl IsInitialized for Mint {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Mint {
    const LEN: usize = 1;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.is_initialized as u8;
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        match src.get(0) {
            Some(val) => {
                let is_initialized = (*val) != 0;
                Ok(Mint { is_initialized })
            }

            None => {
                Err(ProgramError::InvalidAccountData)
            }
        }
    }
}

/// Account data.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of tokens this account holds.
    pub amount: u8,
    pub is_initialized: bool,
}

impl Sealed for Account {}

impl IsInitialized for Account {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Account {
    const LEN: usize = 66;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 66];
        let (mint_dst, owner_dst, amount_dst, is_initialized_dst) = mut_array_refs![dst, 32, 32, 1, 1];
        mint_dst.copy_from_slice(self.mint.as_ref());
        owner_dst.copy_from_slice(self.owner.as_ref());
        amount_dst[0] = self.amount;
        is_initialized_dst[0] = self.is_initialized as u8;
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 66];
        let (mint, owner, amount, is_initialized) = array_refs![src, 32, 32, 1, 1];
        Ok(Account {
            mint: Pubkey::new_from_array(*mint),
            owner: Pubkey::new_from_array(*owner),
            amount: amount[0],
            is_initialized: is_initialized[0] != 0,
        })
    }
}