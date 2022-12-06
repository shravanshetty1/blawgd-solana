use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use super::Instruction;

pub mod execute;
pub mod validate;

#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct InstantiateArgs {}

#[derive(Debug)]
struct InstantiateAccounts {}

pub struct Instantiate {
    program_id: Pubkey,
    accounts: InstantiateAccounts,
    args: InstantiateArgs,
}
impl Instantiate {
    pub fn new<'a>(
        _program_id: Pubkey,
        _accounts: &'a [AccountInfo<'a>],
        _args: InstantiateArgs,
    ) -> Result<Self, ProgramError> {
        todo!()
    }
}

impl Instruction for Instantiate {
    fn validate(&self) -> solana_program::entrypoint::ProgramResult {
        self.validate_instruction()
    }

    fn execute(&mut self) -> solana_program::entrypoint::ProgramResult {
        self.execute_instruction()
    }
}
