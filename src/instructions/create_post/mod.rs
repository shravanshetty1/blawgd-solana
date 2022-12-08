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
pub struct CreatePostArgs {
    parent_post: Option<Pubkey>,
    is_repost: bool,
    content: String,
}

struct CreatePostAccounts<'a, 'b> {
    program_state: &'a AccountInfo<'b>,
    user_account: &'a AccountInfo<'b>,
    post: &'a AccountInfo<'b>,
    account_post: &'a AccountInfo<'b>,
    system_program: &'a AccountInfo<'b>,
    signer: &'a AccountInfo<'b>,
}

pub struct CreatePost<'a, 'b> {
    program_id: Pubkey,
    accounts: CreatePostAccounts<'a, 'b>,
    args: CreatePostArgs,
}
impl<'a, 'b> CreatePost<'a, 'b> {
    pub fn new(
        program_id: Pubkey,
        accounts: &'a [AccountInfo<'b>],
        args: CreatePostArgs,
    ) -> Result<Self, ProgramError> {
        let accounts = &mut accounts.iter();

        let program_state = next_account_info(accounts)?;
        let user_account = next_account_info(accounts)?;
        let post = next_account_info(accounts)?;
        let account_post = next_account_info(accounts)?;
        let system_program = next_account_info(accounts)?;
        let signer = next_account_info(accounts)?;

        Ok(CreatePost {
            program_id,
            accounts: CreatePostAccounts {
                program_state,
                user_account,
                post,
                account_post,
                system_program,
                signer,
            },
            args,
        })
    }
}

impl<'a, 'b> Instruction for CreatePost<'a, 'b> {
    fn validate(&self) -> solana_program::entrypoint::ProgramResult {
        self.validate_instruction()
    }

    fn execute(&mut self) -> solana_program::entrypoint::ProgramResult {
        self.execute_instruction()
    }
}
