use borsh::{BorshDeserialize, BorshSerialize};

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
        b"program_state".to_vec()
    }
}
