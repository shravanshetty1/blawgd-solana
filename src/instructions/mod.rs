use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::entrypoint::ProgramResult;

use self::{instantiate::InstantiateArgs, update_profile::UpdateProfileArgs};

pub mod instantiate;
pub mod update_profile;

pub trait Instruction {
    fn validate(&self) -> ProgramResult;
    fn execute(&mut self) -> ProgramResult;
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum BlawgdInstruction {
    Instantiate(InstantiateArgs),
    UpdateProfile(UpdateProfileArgs),
}
