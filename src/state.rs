use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

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

#[derive(Default, BorshSerialize, BorshDeserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub image: String,
    pub bio: String,
}

impl Profile {
    pub fn space() -> crate::Result<usize> {
        Ok(100 + 200 + 300)
    }

    pub fn seed(address: Pubkey) -> Vec<u8> {
        let res = format!("profile-{address}");
        solana_program::hash::hash(res.as_bytes())
            .to_bytes()
            .to_vec()
    }
}
