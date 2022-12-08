use borsh::{BorshSerialize};
use solana_program::entrypoint::ProgramResult;

use crate::{
    state::{Profile},
    util::create_pda,
};

use super::UpdateProfile;

impl<'a, 'b> UpdateProfile<'a, 'b> {
    pub fn execute_instruction(&self) -> ProgramResult {
        if self.accounts.profile.data.borrow().len() == 0 {
            create_pda(
                &self.program_id,
                Profile::space()?,
                self.accounts.signer,
                self.accounts.profile,
                self.accounts.system_program,
                Profile::seed(*self.accounts.signer.key).as_slice(),
            )?;
        }
        self.args
            .profile
            .serialize(&mut &mut self.accounts.profile.data.borrow_mut()[..])?;

        Ok(())
    }
}
