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
pub struct UpdateFollowingListArgs {
    pub user: Pubkey,
    pub add_operation: bool,
}

struct UpdateFollowingListAccounts<'a, 'b> {
    following_list: &'a AccountInfo<'b>,
    signer_user_account: &'a AccountInfo<'b>,
    to_follow_user_account: &'a AccountInfo<'b>,
    system_program: &'a AccountInfo<'b>,
    signer: &'a AccountInfo<'b>,
}

pub struct UpdateFollowingList<'a, 'b> {
    program_id: Pubkey,
    accounts: UpdateFollowingListAccounts<'a, 'b>,
    args: UpdateFollowingListArgs,
}
impl<'a, 'b> UpdateFollowingList<'a, 'b> {
    pub fn new(
        program_id: Pubkey,
        accounts: &'a [AccountInfo<'b>],
        args: UpdateFollowingListArgs,
    ) -> Result<Self, ProgramError> {
        let accounts = &mut accounts.iter();

        let following_list = next_account_info(accounts)?;
        let signer_user_account = next_account_info(accounts)?;
        let to_follow_user_account = next_account_info(accounts)?;
        let system_program = next_account_info(accounts)?;
        let signer = next_account_info(accounts)?;

        Ok(UpdateFollowingList {
            program_id,
            accounts: UpdateFollowingListAccounts {
                following_list,
                signer_user_account,
                to_follow_user_account,
                system_program,
                signer,
            },
            args,
        })
    }
}

impl<'a, 'b> Instruction for UpdateFollowingList<'a, 'b> {
    fn validate(&self) -> solana_program::entrypoint::ProgramResult {
        self.validate_instruction()
    }

    fn execute(&mut self) -> solana_program::entrypoint::ProgramResult {
        self.execute_instruction()
    }
}
