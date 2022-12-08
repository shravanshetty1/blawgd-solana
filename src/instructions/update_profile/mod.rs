use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::state::Profile;

use super::Instruction;

pub mod execute;
pub mod validate;

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct UpdateProfileArgs {
    pub profile: Profile,
}

struct UpdateProfileAccounts<'a, 'b> {
    profile: &'a AccountInfo<'b>,
    signer: &'a AccountInfo<'b>,
    system_program: &'a AccountInfo<'b>,
}

pub struct UpdateProfile<'a, 'b> {
    program_id: Pubkey,
    accounts: UpdateProfileAccounts<'a, 'b>,
    args: UpdateProfileArgs,
}
impl<'a, 'b> UpdateProfile<'a, 'b> {
    pub fn new(
        program_id: Pubkey,
        accounts: &'a [AccountInfo<'b>],
        args: UpdateProfileArgs,
    ) -> Result<Self, ProgramError> {
        let accounts = &mut accounts.iter();

        let profile = next_account_info(accounts)?;
        let signer = next_account_info(accounts)?;
        let system_program = next_account_info(accounts)?;

        Ok(UpdateProfile {
            program_id,
            accounts: UpdateProfileAccounts {
                profile,
                signer,
                system_program,
            },
            args,
        })
    }
}

impl<'a, 'b> Instruction for UpdateProfile<'a, 'b> {
    fn validate(&self) -> solana_program::entrypoint::ProgramResult {
        self.validate_instruction()
    }

    fn execute(&mut self) -> solana_program::entrypoint::ProgramResult {
        self.execute_instruction()
    }
}
