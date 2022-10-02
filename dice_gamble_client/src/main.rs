use borsh::{BorshDeserialize, BorshSerialize};
use dice_gamble_client::bid_input;
use rand::prelude::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};
use std::fs;
use std::{error::Error, str::FromStr};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Bid {
    pub lamports: u32,
    pub dice: u8,
}
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    //Create new RPC Client
    let rpc_client = RpcClient::new("http://127.0.0.1:8899".to_string());
    //Hard coded program id. Change it after deploying a program.
    let program_id = Pubkey::from_str("6vPhH4abUGWoKAvqn2Ezu5uzsr7cab7eFcvU2zskrjWt")?;

    //Create new Casino keypair if not exist
    let casino_keypair: Keypair;
    if let Ok(keypair) = solana_sdk::signer::keypair::read_keypair_file("casino-keypair.json") {
        casino_keypair = keypair;
        let casino_balance = rpc_client.get_balance(&casino_keypair.pubkey()).await?;
        println!("Casino balance: {}", casino_balance);
    } else {
        casino_keypair = Keypair::new();
        let s = format!("{:?}", casino_keypair.to_bytes());
        fs::write("casino-keypair.json", s)?;
        //Airdrop 100 SOL to new Casino
        let airdrop_sig = rpc_client
            .request_airdrop(&casino_keypair.pubkey(), 1000000000)
            .await?;
        loop {
            let confirmed = rpc_client.confirm_transaction(&airdrop_sig).await?;
            if confirmed {
                break;
            }
        }
        let casino_balance = rpc_client.get_balance(&casino_keypair.pubkey()).await?;
        println!("New Casino was created! Casino balance: {}", casino_balance);
    }

    loop {
        //Enter your bid
        let lamport_bid = bid_input()?;

        //For simplicity generate random number on client side.
        let mut rng = rand::thread_rng();
        let rand_dice_num: u8 = rng.gen_range(1..=6);

        //Read local player keypair. Enter path to your keypair or create new one
        let player_keypair = solana_sdk::signer::keypair::read_keypair_file(
            "Path",
        )?;
        //Data to transfer in instruction
        let data = Bid {
            lamports: lamport_bid,
            dice: rand_dice_num,
        };
        //Local and casino accounts to transfer
        let accounts = vec![
            AccountMeta::new(player_keypair.pubkey(), true),
            AccountMeta::new(casino_keypair.pubkey(), true),
            AccountMeta::new(system_program::id(), false),
        ];
        //Creating an instruction
        let ins = Instruction::new_with_borsh(program_id, &data, accounts);
        //Get latest blockhash to make a transaction
        let blockhash = rpc_client.get_latest_blockhash().await?;
        //Make a new transaction
        let tx = Transaction::new_signed_with_payer(
            &[ins],
            Some(&player_keypair.pubkey()),
            &[&player_keypair, &casino_keypair],
            blockhash,
        );
        //Send and wait for transaction confirmation
        rpc_client.send_and_confirm_transaction(&tx).await?;
        let balance = rpc_client.get_balance(&player_keypair.pubkey()).await?;
        if rand_dice_num != 6 {
            println!("You lose :( Your balance: {}", balance);
        } else {
            println!("You win! Congratulations Your balance: {}", balance);
        }
        println!("Let's play again? Ctrl+C to exit");
    }
    Ok(())
}
