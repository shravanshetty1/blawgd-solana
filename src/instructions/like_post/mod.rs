use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use super::Instruction;

pub mod execute;
pub mod validate;

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct LikePostArgs {}

struct LikePostAccounts<'a, 'b> {
    post_account: &'a AccountInfo<'b>,
    post_user_interaction_status: &'a AccountInfo<'b>,
    system_program: &'a AccountInfo<'b>,
    signer: &'a AccountInfo<'b>,
}

pub struct LikePost<'a, 'b> {
    program_id: Pubkey,
    accounts: LikePostAccounts<'a, 'b>,
    args: LikePostArgs,
}
impl<'a, 'b> LikePost<'a, 'b> {
    pub fn new(
        program_id: Pubkey,
        accounts: &'a [AccountInfo<'b>],
        args: LikePostArgs,
    ) -> Result<Self, ProgramError> {
        let accounts = &mut accounts.iter();

        let post_account = next_account_info(accounts)?;
        let post_user_interaction_status = next_account_info(accounts)?;
        let system_program = next_account_info(accounts)?;
        let signer = next_account_info(accounts)?;

        Ok(LikePost {
            program_id,
            accounts: LikePostAccounts {
                post_account,
                post_user_interaction_status,
                system_program,
                signer,
            },
            args,
        })
    }
}

impl<'a, 'b> Instruction for LikePost<'a, 'b> {
    fn validate(&self) -> solana_program::entrypoint::ProgramResult {
        self.validate_instruction()
    }

    fn execute(&mut self) -> solana_program::entrypoint::ProgramResult {
        self.execute_instruction()
    }
}
