use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::{fs, str::FromStr};

#[tokio::main]
async fn main() {
    // Program ID
    let program_id = Pubkey::from_str("Bcs3x7JCccn5oWsQvVEpohGgVyYEZXGVJh8xns5yNRTk").unwrap();

    // Connect to the Solana devnet
    let rpc_url = String::from("https://api.devnet.solana.com");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // // Generate a new keypair for the payer
    // let payer = Keypair::new();

    // // Request airdrop
    // let airdrop_amount = 1_000_000_000; // 1 SOL
    // let signature = client
    //     .request_airdrop(&payer.pubkey(), airdrop_amount)
    //     .expect("Failed to request airdrop");

    // // Wait for airdrop confirmation
    // loop {
    //     let confirmed = client.confirm_transaction(&signature).unwrap();
    //     if confirmed {
    //         break;
    //     }
    // }

    let key_file = "/home/ssuchichan/.config/solana/id.json";
    let data = fs::read_to_string(key_file).unwrap();
    let key_bytes: Vec<u8> = serde_json::from_str(&data).unwrap();
    let payer = Keypair::from_bytes(&key_bytes).unwrap();

    // Create the instruction
    let instruction = Instruction::new_with_borsh(program_id, &(), vec![]);

    // Add the instruction to new transaction
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], client.get_latest_blockhash().unwrap());

    // Send and confirm the transaction
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
}
