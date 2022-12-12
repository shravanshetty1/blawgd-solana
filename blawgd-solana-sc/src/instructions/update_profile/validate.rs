use solana_program::entrypoint::ProgramResult;

use super::UpdateProfile;

impl<'a, 'b> UpdateProfile<'a, 'b> {
    pub fn validate_instruction(&self) -> ProgramResult {
        Ok(())
    }
}
