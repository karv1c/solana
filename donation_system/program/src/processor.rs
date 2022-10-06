use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{
    error::DonateError, id, instruction::DonationInstruction, state::Storage, STORAGE_SEED,
};

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("instruction_data: {:?}", instruction_data);
        let instruction = DonationInstruction::try_from_slice(instruction_data)?;
        match instruction {
            DonationInstruction::Donate { lamports } => Self::process_donation(accounts, lamports),
            DonationInstruction::Withdraw { lamports } => {
                Self::process_withdraw(accounts, lamports)
            },
            DonationInstruction::Initialize => Self::initialize(accounts)
        }
    }
    fn process_donation(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
        //Accounts
        let acc_iter = &mut accounts.iter();
        let user_info = next_account_info(acc_iter)?;
        let donating_info = next_account_info(acc_iter)?;
        let storage_info = next_account_info(acc_iter)?;
        //Check storage pubkey
        if !Storage::is_ok_storage_pubkey(storage_info.key) {
            return Err(DonateError::WrongStoragePDA.into());
        }
        //Check donator is signer
        if !user_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let mut storage = try_from_slice_unchecked::<Storage>(&storage_info.data.borrow())?;

        if storage.donating_acc != donating_info.key.to_bytes() && storage.donating_acc != [0; 32] {
            return Err(DonateError::DonatingRequired.into());
        }
        if let Some(value) = storage.map.get(&*user_info.key).as_deref() {
            storage.map.insert(*user_info.key, lamports + *value);
        } else {
            storage.map.insert(*user_info.key, lamports);
        }
        //Print all donations
        for (key, value) in &storage.map {
            msg!("{:?}:{:?}", key, value);
        }

        let _ = storage.serialize(&mut &mut storage_info.data.borrow_mut()[..]);

        msg!("List was updated");

        invoke(
            &system_instruction::transfer(user_info.key, donating_info.key, lamports),
            &[user_info.clone(), donating_info.clone()],
        )?;
        msg!(
            "Donate {} lamports from {:?} to {:?}: done",
            lamports,
            user_info.key,
            donating_info.key
        );
        Ok(())
    }
    fn process_withdraw(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
        //Accounts
        let acc_iter = &mut accounts.iter();
        let donating_info = next_account_info(acc_iter)?;
        let donating_wallet_info = next_account_info(acc_iter)?;
        let storage_info = next_account_info(acc_iter)?;
        let system_program_info = next_account_info(acc_iter)?;
        //Check signer
        if !donating_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        //Check storage key
        if !Storage::is_ok_storage_pubkey(storage_info.key) {
            return Err(DonateError::WrongStoragePDA.into());
        }
        //Check donating acc
        let storage = try_from_slice_unchecked::<Storage>(&storage_info.data.borrow())?;
        if storage.donating_acc != donating_info.key.to_bytes() {
            return Err(DonateError::WrongDonatingAccount.into());
        }
        //Withdraw
        invoke(
            &system_instruction::transfer(donating_info.key, donating_wallet_info.key, lamports),
            &[donating_info.clone(), donating_wallet_info.clone(), system_program_info.clone()],
        )?;
        msg!(
            "Withdraw {} lamports from {:?} to {:?}: done",
            lamports,
            donating_info.key,
            donating_wallet_info.key
        );
        Ok(())
    }
    fn initialize(accounts: &[AccountInfo]) -> ProgramResult {
        let acc_iter = &mut accounts.iter();
        let donating_info = next_account_info(acc_iter)?;
        let storage_info = next_account_info(acc_iter)?;
        let rent_info = next_account_info(acc_iter)?;
        let system_program_info = next_account_info(acc_iter)?;
        let (storage_pubkey, bump_seed) = Storage::get_storage_pubkey_with_bump();
        if !Storage::is_ok_storage_pubkey(storage_info.key) {
            return Err(DonateError::WrongStoragePDA.into());
        }
        //Check donator is signer
        if !donating_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if storage_info.data_is_empty() {
            msg!("Creating storage account");
            let space = Storage::LEN;

            let rent = &Rent::from_account_info(rent_info)?;
            msg!("Rent defined");
            let lamports = rent.minimum_balance(space);
            msg!("Lamports to pay: {}", lamports);

            let signer_seeds: &[&[_]] = &[STORAGE_SEED.as_bytes(), &[bump_seed]];
            invoke_signed(
                &system_instruction::create_account(
                    donating_info.key,
                    &storage_pubkey,
                    lamports,
                    space as u64,
                    &id(),
                ),
                &[
                    donating_info.clone(),
                    storage_info.clone(),
                    system_program_info.clone(),
                ],
                &[&signer_seeds],
            )?;
        }
        let mut storage = try_from_slice_unchecked::<Storage>(&storage_info.data.borrow())?;
        storage.donating_acc = donating_info.key.to_bytes();
        storage.map = BTreeMap::<Pubkey, u64>::new();
        let _ = storage.serialize(&mut &mut storage_info.data.borrow_mut()[..]);

        msg!("List is initialized");
        Ok(())
    }
}

