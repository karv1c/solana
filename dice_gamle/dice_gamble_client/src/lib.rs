use std::error::Error;

use std::io;

pub fn bid_input() -> Result<u32, Box<dyn Error>> {
    let mut lamport_bid_input = String::new();
    let lamport_bid: u32;
    println!("Enter your bid: ");
    loop {
        let stdin = io::stdin();
        stdin.read_line(&mut lamport_bid_input)?;
        match lamport_bid_input.trim().parse::<u32>() {
            Ok(bid) => {
                lamport_bid = bid;
                break;
            }
            Err(_) => {
                lamport_bid_input = "".to_string();
                println!("Please, enter an integer:")
            }
        }
    }
    println!("Your bid is: {} lamports", lamport_bid);
    println!("Please, wait for result...");
    Ok(lamport_bid)
}
