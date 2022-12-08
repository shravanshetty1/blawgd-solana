use solana_program::entrypoint::ProgramResult;

use super::LikePost;

impl<'a, 'b> LikePost<'a, 'b> {
    pub fn validate_instruction(&self) -> ProgramResult {
        Ok(())
    }
}
