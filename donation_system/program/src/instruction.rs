use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum DonationInstruction {
    /// Make a donation.
    /// Accounts:
    /// 0. `[signer]` donator account
    /// 1. `[writable]` donating account
    /// 2. `[writable]` storage account, PDA
    Donate { lamports: u64 },

    /// Withdraw donation.
    /// Accounts:
    /// 0. `[signer]` donating account
    /// 1. `[writable]` donating withdraw wallet account
    /// 2. `[]` storage account, PDA
    /// 3. `[]` system
    Withdraw { lamports: u64 },

    /// Initialize.
    /// Accounts:
    /// 0. `[signer]` donating account
    /// 1. `[writable]` donating withdraw wallet account
    /// 2. `[writable]` storage account, PDA
    /// 3. `[]` rent
    /// 4. `[]` system
    Initialize
}
