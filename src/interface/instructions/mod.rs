pub mod my_instruction;
pub use my_instruction::*;

#[repr(u8)]
pub enum ProgramInstruction {
    MyInstruction,
}

impl Into<u8> for ProgramInstruction {
    fn into(self) -> u8 {
        self as u8
    }
}
