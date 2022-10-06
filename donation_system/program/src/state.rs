use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{id, STORAGE_SEED};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Storage {
    /// Donating pubkey
    pub donating_acc: [u8; 32],

    // List of all donations
    pub map: BTreeMap<Pubkey, u64>,
}

impl Storage {
    pub fn get_storage_pubkey_with_bump() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[STORAGE_SEED.as_bytes()], &id())
    }

    pub fn get_storage_pubkey() -> Pubkey {
        let (pubkey, _) = Self::get_storage_pubkey_with_bump();
        pubkey
    }
    pub fn is_ok_storage_pubkey(storage_pubkey: &Pubkey) -> bool {
        let (pubkey, _) = Self::get_storage_pubkey_with_bump();
        pubkey.to_bytes() == storage_pubkey.to_bytes()
    }
    pub const LEN: usize = 32 + (4 + (10 * 40));
}
