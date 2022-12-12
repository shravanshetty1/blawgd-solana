use borsh::BorshSerialize;
use solana_program::entrypoint::ProgramResult;

use crate::{state::program_state::ProgramState, util::create_pda};

use super::Instantiate;

impl<'a, 'b> Instantiate<'a, 'b> {
    pub fn execute_instruction(&self) -> ProgramResult {
        create_pda(
            &self.program_id,
            ProgramState::space()?,
            self.accounts.signer,
            self.accounts.program_state,
            self.accounts.system_program,
            ProgramState::seed().as_slice(),
        )?;
        let state = ProgramState { post_count: 0 };
        state.serialize(&mut &mut self.accounts.program_state.data.borrow_mut()[..])?;

        Ok(())
    }
}
