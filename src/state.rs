use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct State {
    pub post_count: u128,
}

impl State {
    pub fn space() -> crate::Result<usize> {
        Ok(State {
            post_count: u128::MAX,
        }
        .try_to_vec()?
        .len())
    }

    pub fn seed() -> Vec<u8> {
        b"program_state".to_vec()
    }
}
