use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::entrypoint::ProgramResult;

use crate::{state::account::UserAccount, util::create_pda};

use super::UpdateProfile;

impl<'a, 'b> UpdateProfile<'a, 'b> {
    pub fn execute_instruction(&self) -> ProgramResult {
        let mut user_account_state = if self.accounts.account_state.data.borrow().len() > 0 {
            UserAccount::deserialize(&mut &**self.accounts.account_state.data.borrow())?
        } else {
            create_pda(
                &self.program_id,
                UserAccount::space()?,
                self.accounts.signer,
                self.accounts.account_state,
                self.accounts.system_program,
                UserAccount::seed(*self.accounts.signer.key).as_slice(),
            )?;
            UserAccount::default()
        };
        user_account_state.profile = self.args.profile.clone();
        user_account_state
            .serialize(&mut &mut self.accounts.account_state.data.borrow_mut()[..])?;

        Ok(())
    }
}
