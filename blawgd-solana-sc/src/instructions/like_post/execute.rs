use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::entrypoint::ProgramResult;

use crate::{
    state::post::{Post, PostUserInteractionStatus},
    util::create_pda,
};

use super::LikePost;

impl<'a, 'b> LikePost<'a, 'b> {
    pub fn execute_instruction(&self) -> ProgramResult {
        let mut post = Post::deserialize(&mut &**self.accounts.post_account.data.borrow())?;
        let mut post_user_interaction_status = if self
            .accounts
            .post_user_interaction_status
            .data
            .borrow()
            .len()
            > 0
        {
            PostUserInteractionStatus::deserialize(
                &mut &**self.accounts.post_user_interaction_status.data.borrow(),
            )?
        } else {
            create_pda(
                &self.program_id,
                PostUserInteractionStatus::space()?,
                self.accounts.signer,
                self.accounts.post_user_interaction_status,
                self.accounts.system_program,
                PostUserInteractionStatus::seed(
                    *self.accounts.post_account.key,
                    *self.accounts.signer.key,
                )
                .as_slice(),
            )?;
            PostUserInteractionStatus::default()
        };

        post.like_count += 1;
        post_user_interaction_status.liked = true;

        post.serialize(&mut &mut self.accounts.post_account.data.borrow_mut()[..])?;
        post_user_interaction_status.serialize(
            &mut &mut self.accounts.post_user_interaction_status.data.borrow_mut()[..],
        )?;

        Ok(())
    }
}
