extern crate borsh;
extern crate solana_program;

pub mod instructions;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

solana_program::declare_id!("GE15QxJUB1NeLGzRM4bUaRexQGjdHafmAuAvh5buM8j8");
