use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ProgramState {
    pub post_count: u128,
}

impl ProgramState {
    pub fn space() -> crate::Result<usize> {
        Ok(32)
    }

    pub fn seed() -> Vec<u8> {
        let res = b"program_state";
        solana_program::hash::hash(res).to_bytes().to_vec()
    }
}
