use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::entrypoint::ProgramResult;

use crate::{
    state::{
        account::{AccountPost, UserAccount},
        post::Post,
    },
    util::create_pda,
};

use super::CreatePost;

impl<'a, 'b> CreatePost<'a, 'b> {
    pub fn execute_instruction(&self) -> ProgramResult {
        let mut user_account_state =
            UserAccount::deserialize(&mut &**self.accounts.user_account.data.borrow())?;
        let mut program_state =
            UserAccount::deserialize(&mut &**self.accounts.program_state.data.borrow())?;
        let post = Post {
            creator: *self.accounts.signer.key,
            content: self.args.content.clone(),
            parent_post: self.args.parent_post,
            is_repost: self.args.is_repost,
            like_count: 0,
            comment_count: 0,
            repost_count: 0,
        };
        let account_post = AccountPost {
            post: *self.accounts.post.key,
        };
        user_account_state.post_count += 1;
        program_state.post_count += 1;

        create_pda(
            &self.program_id,
            Post::space()?,
            self.accounts.signer,
            self.accounts.post,
            self.accounts.system_program,
            Post::seed(program_state.post_count).as_slice(),
        )?;
        create_pda(
            &self.program_id,
            AccountPost::space()?,
            self.accounts.signer,
            self.accounts.account_post,
            self.accounts.system_program,
            AccountPost::seed(user_account_state.post_count).as_slice(),
        )?;
        user_account_state.serialize(&mut &mut self.accounts.user_account.data.borrow_mut()[..])?;
        program_state.serialize(&mut &mut self.accounts.program_state.data.borrow_mut()[..])?;
        post.serialize(&mut &mut self.accounts.post.data.borrow_mut()[..])?;
        account_post.serialize(&mut &mut self.accounts.account_post.data.borrow_mut()[..])?;

        Ok(())
    }
}
