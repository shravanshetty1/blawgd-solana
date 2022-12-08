use solana_program::entrypoint::ProgramResult;

use super::CreatePost;

impl<'a, 'b> CreatePost<'a, 'b> {
    pub fn validate_instruction(&self) -> ProgramResult {
        Ok(())
    }
}
