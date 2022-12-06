use solana_program::entrypoint::ProgramResult;

use super::Instantiate;

impl Instantiate<'_> {
    pub fn validate_instruction(&self) -> ProgramResult {
        Ok(())
    }
}
