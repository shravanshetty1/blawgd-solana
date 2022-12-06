

use borsh::{BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey,
    sysvar::Sysvar,
};

use crate::state::State;

use super::Instantiate;

impl Instantiate<'_> {
    pub fn execute_instruction(&self) -> ProgramResult {
        create_pda(
            &self.program_id,
            State::space()?,
            self.accounts.creator,
            self.accounts.program_state,
            self.accounts.system_account,
            State::seed().as_slice(),
        )?;
        let state = State { post_count: 0 };
        state.serialize(&mut &mut self.accounts.program_state.data.borrow_mut()[..])?;

        Ok(())
    }
}

pub fn create_pda<'a>(
    program_id: &Pubkey,
    space: usize,
    creator: &AccountInfo<'a>,
    pda: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    seed: &[u8],
) -> ProgramResult {
    let rent = solana_program::sysvar::rent::Rent::get()?.minimum_balance(space);

    let ix = solana_program::system_instruction::create_account(
        creator.key,
        pda.key,
        rent,
        space as u64,
        program_id,
    );

    let (_, nonce) = Pubkey::find_program_address(&[seed], program_id);
    invoke_signed(
        &ix,
        &[creator.clone(), pda.clone(), system_program.clone()],
        &[&[seed, &[nonce]]],
    )
}
