use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

#[derive(Default, BorshSerialize, borsh::BorshDeserialize, Clone)]
pub struct Post {
    pub parent_post: Option<Pubkey>,
    pub creator: Pubkey,
    pub content: String,
    pub is_repost: bool,
    pub like_count: u128,
    pub comment_count: u128,
    pub repost_count: u128,
}

impl Post {
    pub fn space() -> crate::Result<usize> {
        Ok(32 + 32 + 280 + 1 + 16 + 16 + 16)
    }
    pub fn seed(post_num: u128) -> Vec<u8> {
        let res = format!("post-{post_num}");
        solana_program::hash::hash(res.as_bytes())
            .to_bytes()
            .to_vec()
    }
}

#[derive(Default, BorshSerialize, borsh::BorshDeserialize, Clone)]
pub struct PostUserInteractionStatus {
    pub liked: bool,
    pub commented: bool,
    pub reposted: bool,
}

impl PostUserInteractionStatus {
    pub fn space() -> crate::Result<usize> {
        Ok(6)
    }
    pub fn seed(post: Pubkey, user: Pubkey) -> Vec<u8> {
        let res = format!("post-user-interaction-status-{post}-{user}");
        solana_program::hash::hash(res.as_bytes())
            .to_bytes()
            .to_vec()
    }
}
