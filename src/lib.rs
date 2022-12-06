use solana_program::program_error::ProgramError;

pub mod instructions;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

type Result<T> = core::result::Result<T, ProgramError>;

solana_program::declare_id!("GE15QxJUB1NeLGzRM4bUaRexQGjdHafmAuAvh5buM8j8");
