use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //msg!("Hello Solana! (From Rust!)");
    msg!("Hello World!");
    Ok(())
}

#[cfg(test)]
mod test {
    use solana_program_test::*;
    use solana_sdk::{
        instruction::Instruction, pubkey::Pubkey, signature::Signer, transaction::Transaction,
    };

    #[tokio::test]
    async fn test_hello_solana() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::default();
        program_test.add_program("program", program_id, None);
        let (banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create instruction
        let instruction = Instruction {
            program_id,
            accounts: vec![],
            data: vec![],
        };

        // Create transaction with instruction
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));

        // Sign transaction
        transaction.sign(&[&payer], recent_blockhash);

        let transaction_result = banks_client.process_transaction(transaction).await;
        assert!(transaction_result.is_ok());
    }
}
