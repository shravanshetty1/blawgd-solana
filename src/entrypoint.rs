use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use crate::instructions::{
    instantiate::Instantiate, update_profile::UpdateProfile, BlawgdInstruction, Instruction,
};

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: &[u8],
) -> ProgramResult {
    let instruction_type = BlawgdInstruction::try_from_slice(args)?;

    let mut instruction: Box<dyn Instruction> = match instruction_type {
        BlawgdInstruction::Instantiate(args) => {
            Box::new(Instantiate::new(*program_id, accounts, args)?)
        }
        BlawgdInstruction::UpdateProfile(args) => {
            Box::new(UpdateProfile::new(*program_id, accounts, args)?)
        }
    };

    instruction.validate()?;
    instruction.execute()
}
