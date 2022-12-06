use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::entrypoint::ProgramResult;

use self::instantiate::InstantiateArgs;

pub mod instantiate;

pub trait Instruction {
    fn validate(&self) -> ProgramResult;
    fn execute(&mut self) -> ProgramResult;
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum BlawgdInstruction {
    Instantiate(InstantiateArgs),
}
