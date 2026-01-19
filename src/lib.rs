#![no_std]

pub mod interface;
pub mod states; 


use pinocchio::{
  AccountView,
  Address,
  entrypoint,
  ProgramResult
};

solana_address::declare_id!("GJJuYV5QA1Lt9Ht5rdmVgvXdgjTJDe7nfJQ47YLvdstV");

use core::mem::MaybeUninit;
const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();
#[inline(always)]
pub fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {

    // SAFETY:
    // - Pointers are of alignment 1,
    // - the length will not exceed either pointer.
    unsafe {
        core::ptr::copy_nonoverlapping(
            source.as_ptr(), 
            destination.as_mut_ptr() as *mut u8, 
            destination.len().min(source.len())
        );
    }
}

use solana_program_log::log;


entrypoint!(process_instruction);

pub fn process_instruction(
  program_id: &Address,
  accounts: &[AccountView],
  instruction_data: &[u8],
) -> ProgramResult {
  log("Hello from my pinocchio program!");
  Ok(())
}
