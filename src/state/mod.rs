use borsh::{BorshDeserialize, BorshSerialize};

pub mod account;
pub mod post;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ProgramState {
    pub post_count: u128,
}

impl ProgramState {
    pub fn space() -> crate::Result<usize> {
        Ok(ProgramState {
            post_count: u128::MAX,
        }
        .try_to_vec()?
        .len())
    }

    pub fn seed() -> Vec<u8> {
        let res = b"program_state";
        solana_program::hash::hash(res).to_bytes().to_vec()
    }
}
