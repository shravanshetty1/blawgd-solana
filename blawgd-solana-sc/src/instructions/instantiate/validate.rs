use solana_program::entrypoint::ProgramResult;

use super::Instantiate;

impl<'a, 'b> Instantiate<'a, 'b> {
    pub fn validate_instruction(&self) -> ProgramResult {
        Ok(())
    }
}
