use solana_program::program_error::ProgramError;

pub mod instructions;
pub mod state;
pub mod util;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub type Result<T> = core::result::Result<T, ProgramError>;

solana_program::declare_id!("GE15QxJUB1NeLGzRM4bUaRexQGjdHafmAuAvh5buM8j8");
