use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
};

// declare and export the program's entrypoint
entrypoint!(process_instruction);
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Bid {
    pub lamports: u32,
    pub dice: u8,
}
// program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let bid = Bid::try_from_slice(instruction_data)?.lamports;
    let dice = Bid::try_from_slice(instruction_data)?.dice;
    msg!("Your bid is: {}", bid);
    msg!("Player throw {}", dice);
    let acc_iter = &mut accounts.iter();
    let player_info = next_account_info(acc_iter)?;
    let casino_info = next_account_info(acc_iter)?;
    if dice != 6 {
        invoke(
            &system_instruction::transfer(player_info.key, casino_info.key, bid.into()),
            &[player_info.clone(), casino_info.clone()],
        )?;
        msg!(
            "transfer {} lamports from {:?} to {:?}",
            bid,
            player_info.key,
            casino_info.key
        );
    } else {
        invoke(
            &system_instruction::transfer(casino_info.key, player_info.key, (bid * 5).into()),
            &[casino_info.clone(), player_info.clone()],
        )?;
        msg!(
            "transfer {} lamports from {:?} to {:?}",
            bid * 5,
            casino_info.key,
            player_info.key
        );
    }

    // gracefully exit the program
    Ok(())
}
