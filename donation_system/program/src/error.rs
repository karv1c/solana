use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum DonateError {
    #[error("Donating account is required")]
    DonatingRequired,

    #[error("Wrong storage PDA")]
    WrongStoragePDA,

    #[error("Wrong donating account")]
    WrongDonatingAccount,
}

impl From<DonateError> for ProgramError {
    fn from(e: DonateError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
