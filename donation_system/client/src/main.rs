use std::{error::Error, str::FromStr};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer,
    system_program,
    sysvar::{self},
    transaction::Transaction,
};
#[derive(BorshSerialize, BorshDeserialize)]
enum DonationInstruction {
    Donate { lamports: u64 },
    Withdraw { lamports: u64 },
    Initialize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_client = RpcClient::new("http://127.0.0.1:8899".to_string());
    let program_id = Pubkey::from_str("CVwg4yLYEW3WxHNPYo3ZtTcvYPnevm99DNwj8KeoVaAE")?;

    let donating_keypair =
        solana_sdk::signer::keypair::read_keypair_file("../localnet/admin.json")?;
    let donating_wallet_keypair =
        solana_sdk::signer::keypair::read_keypair_file("../localnet/admin_wallet.json")?;
    let client_a_keypair =
        solana_sdk::signer::keypair::read_keypair_file("../localnet/clientA.json")?;
    let client_b_keypair =
        solana_sdk::signer::keypair::read_keypair_file("../localnet/clientB.json")?;
    let (storage_pubkey, _) = Pubkey::find_program_address(&["storage".as_bytes()], &program_id);

    let rent = sysvar::rent::id();

    //Initialize List
    let data = DonationInstruction::Initialize;
    let accounts = vec![
        AccountMeta::new(donating_keypair.pubkey(), true),
        AccountMeta::new(storage_pubkey, false),
        AccountMeta::new(rent, false),
        AccountMeta::new(system_program::id(), false),
    ];
    let ix = Instruction::new_with_borsh(program_id, &data, accounts);
    let blockhash = rpc_client.get_latest_blockhash().await?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&donating_keypair.pubkey()),
        &[&donating_keypair],
        blockhash,
    );
    rpc_client.send_and_confirm_transaction(&tx).await?;
    //Donate with Client A
    let data = DonationInstruction::Donate { lamports: 10000 };
    let accounts = vec![
        AccountMeta::new(client_a_keypair.pubkey(), true),
        AccountMeta::new(donating_keypair.pubkey(), false),
        AccountMeta::new(storage_pubkey, false),
        AccountMeta::new(rent, false),
        AccountMeta::new(system_program::id(), false),
    ];
    let ix = Instruction::new_with_borsh(program_id, &data, accounts);
    let blockhash = rpc_client.get_latest_blockhash().await?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&client_a_keypair.pubkey()),
        &[&client_a_keypair],
        blockhash,
    );
    rpc_client.send_and_confirm_transaction(&tx).await?;

   //Donate with Client B
   let data = DonationInstruction::Donate { lamports: 10000 };
    let accounts = vec![
        AccountMeta::new(client_b_keypair.pubkey(), true),
        AccountMeta::new(donating_keypair.pubkey(), false),
        AccountMeta::new(storage_pubkey, false),
        AccountMeta::new(rent, false),
        AccountMeta::new(system_program::id(), false),
    ];
    let ix = Instruction::new_with_borsh(program_id, &data, accounts);
    let blockhash = rpc_client.get_latest_blockhash().await?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&client_b_keypair.pubkey()),
        &[&client_b_keypair],
        blockhash,
    );
    rpc_client.send_and_confirm_transaction(&tx).await?;

    //Withdraw to wallet
    let accounts3 = vec![
        AccountMeta::new(donating_keypair.pubkey(), true),
        AccountMeta::new(donating_wallet_keypair.pubkey(), false),
        AccountMeta::new(storage_pubkey, false),
        AccountMeta::new(system_program::id(), false),
    ];
    let data = DonationInstruction::Withdraw { lamports: 10000 };
    let ix = Instruction::new_with_borsh(program_id, &data, accounts3);
    let blockhash = rpc_client.get_latest_blockhash().await?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&donating_keypair.pubkey()),
        &[&donating_keypair],
        blockhash,
    );
    rpc_client.send_and_confirm_transaction(&tx).await?;
 
    Ok(())
}
