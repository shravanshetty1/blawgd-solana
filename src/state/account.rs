use std::collections::BTreeSet;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Default, BorshSerialize, BorshDeserialize, Clone)]
pub struct UserAccount {
    pub profile: Profile,
    pub post_count: u128,
    pub follower_count: u128,
    pub following_count: u128,
}

impl UserAccount {
    pub fn space() -> crate::Result<usize> {
        Ok(Profile::space()? + 16 + 16)
    }
    pub fn seed(address: Pubkey) -> Vec<u8> {
        let res = format!("account-{address}");
        solana_program::hash::hash(res.as_bytes())
            .to_bytes()
            .to_vec()
    }
}

#[derive(Default, BorshSerialize, BorshDeserialize, Clone)]
pub struct UserFollowingList {
    pub list: BTreeSet<Pubkey>,
}

impl UserFollowingList {
    pub fn space() -> crate::Result<usize> {
        Ok(32 * 50)
    }
    pub fn seed(address: Pubkey) -> Vec<u8> {
        let res = format!("following-list-{address}");
        solana_program::hash::hash(res.as_bytes())
            .to_bytes()
            .to_vec()
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
}

#[derive(Default, BorshSerialize, BorshDeserialize, Clone)]
pub struct AccountPost {
    pub post: Pubkey,
}

impl AccountPost {
    pub fn space() -> crate::Result<usize> {
        Ok(32)
    }
    pub fn seed(acc_post_num: u128) -> Vec<u8> {
        let res = format!("account-post-{acc_post_num}");
        solana_program::hash::hash(res.as_bytes())
            .to_bytes()
            .to_vec()
    }
}
