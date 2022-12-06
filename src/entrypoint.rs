
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction<'a>(
    _program_id: &Pubkey,
    _accounts: &'a [AccountInfo<'a>],
    _args: &[u8],
) -> ProgramResult {
    ProgramResult::Ok(())
}
