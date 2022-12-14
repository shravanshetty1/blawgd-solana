use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{entrypoint::ProgramResult, msg, program_error::ProgramError};

use crate::{
    state::{
        account::{AccountPost, UserAccount},
        post::{Comment, Post, PostUserInteractionStatus},
        program_state::ProgramState,
    },
    util::create_pda,
};

use super::CreatePost;

impl<'a, 'b> CreatePost<'a, 'b> {
    pub fn execute_instruction(&self) -> ProgramResult {
        let mut user_account_state =
            UserAccount::deserialize(&mut &**self.accounts.signer_account.data.borrow())?;
        let mut program_state =
            ProgramState::deserialize(&mut &**self.accounts.program_state.data.borrow())?;
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
            AccountPost::seed(post.creator, user_account_state.post_count).as_slice(),
        )?;
        user_account_state
            .serialize(&mut &mut self.accounts.signer_account.data.borrow_mut()[..])?;
        program_state.serialize(&mut &mut self.accounts.program_state.data.borrow_mut()[..])?;
        post.serialize(&mut &mut self.accounts.post.data.borrow_mut()[..])?;
        account_post.serialize(&mut &mut self.accounts.account_post.data.borrow_mut()[..])?;

        if let Some(parent_post_acc) = self.accounts.parent_post {
            if let Some(parent_post_user_interaction_status_acc) =
                self.accounts.parent_post_user_interaction_status
            {
                let mut parent_post_user_interaction_status =
                    if parent_post_user_interaction_status_acc.data.borrow().len() > 0 {
                        PostUserInteractionStatus::deserialize(
                            &mut &**parent_post_user_interaction_status_acc.data.borrow(),
                        )?
                    } else {
                        create_pda(
                            &self.program_id,
                            PostUserInteractionStatus::space()?,
                            self.accounts.signer,
                            parent_post_user_interaction_status_acc,
                            self.accounts.system_program,
                            PostUserInteractionStatus::seed(
                                *parent_post_acc.key,
                                *self.accounts.signer.key,
                            )
                            .as_slice(),
                        )?;
                        PostUserInteractionStatus::default()
                    };

                let mut parent_post = Post::deserialize(&mut &**parent_post_acc.data.borrow())?;
                if post.is_repost {
                    parent_post_user_interaction_status.reposted = true;
                    parent_post.repost_count += 1;
                } else {
                    parent_post_user_interaction_status.commented = true;
                    parent_post.comment_count += 1;

                    let comment_acc = self.accounts.comment.ok_or_else(|| {
                        msg!("could not find comment account for comment post");
                        ProgramError::InvalidAccountData
                    })?;
                    create_pda(
                        &self.program_id,
                        Comment::space()?,
                        self.accounts.signer,
                        comment_acc,
                        self.accounts.system_program,
                        Comment::seed(*parent_post_acc.key, parent_post.comment_count)
                            .as_slice(),
                    )?;
                    let comment_state = Comment {
                        post: *self.accounts.post.key,
                    };
                    comment_state.serialize(&mut &mut comment_acc.data.borrow_mut()[..])?;
                }
                parent_post_user_interaction_status.serialize(
                    &mut &mut parent_post_user_interaction_status_acc.data.borrow_mut()[..],
                )?;
                parent_post.serialize(&mut &mut parent_post_acc.data.borrow_mut()[..])?;
            } else {
                msg!("parent post user interaction status is invalid for repost/comment");
                return Err(ProgramError::InvalidAccountData);
            }
        }

        Ok(())
    }
}
