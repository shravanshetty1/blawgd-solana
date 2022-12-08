use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{entrypoint::ProgramResult};

use self::{
    create_post::{CreatePostArgs},
    instantiate::{InstantiateArgs},
    update_following_list::{UpdateFollowingListArgs},
    update_profile::{UpdateProfileArgs},
};

pub mod create_post;
pub mod instantiate;
pub mod like_post;
pub mod update_following_list;
pub mod update_profile;

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
