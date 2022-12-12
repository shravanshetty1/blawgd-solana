use solana_program::entrypoint::ProgramResult;

use super::UpdateFollowingList;

impl<'a, 'b> UpdateFollowingList<'a, 'b> {
    pub fn validate_instruction(&self) -> ProgramResult {
        Ok(())
    }
}
