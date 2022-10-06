pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub use solana_program;

solana_program::declare_id!("CVwg4yLYEW3WxHNPYo3ZtTcvYPnevm99DNwj8KeoVaAE");
pub const STORAGE_SEED: &str = "storage";
