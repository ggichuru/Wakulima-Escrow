use thiserror::Error;

use solana_program::program_error::ProgramError;

// Define error type
#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    ///
    #[error("Invlaid Instruction")]
    InvalidInstruction,

    #[error("Not Rent Expempt")]
    NotRentExempt,
}

/// Convert an EscrowError into a ProgramError
///
/// Implementing a generic trait, specifically the From trait which the `?` operator wants to use
impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// WHY CONVERT to program error
// Because the entrypoint returns a Result of either nothing or a Program error
