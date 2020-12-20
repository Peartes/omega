#[macro_use]
pub mod error;

pub mod processor;
pub mod state;
pub mod instruction;


use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, entrypoint, pubkey::Pubkey,
};

use crate::processor::Processor;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}