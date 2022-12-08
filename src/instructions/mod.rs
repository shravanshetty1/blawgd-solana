use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::entrypoint::ProgramResult;

use self::{instantiate::InstantiateArgs, update_profile::UpdateProfileArgs, create_post::CreatePostArgs, update_following_list::UpdateFollowingListArgs};

pub mod instantiate;
pub mod update_profile;
pub mod create_post;
pub mod update_following_list;

pub trait Instruction {
    fn validate(&self) -> ProgramResult;
    fn execute(&mut self) -> ProgramResult;
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum BlawgdInstruction {
    Instantiate(InstantiateArgs),
    UpdateProfile(UpdateProfileArgs),
    CreatePost(CreatePostArgs),
    UpdateFollowingList(UpdateFollowingListArgs),
}
