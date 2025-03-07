import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
  } from '@solana/web3.js';
import * as fs from "fs";
import path from 'path';

const PROGRAM_KEYPAIR_PATH = path.join(
    path.resolve(__dirname, '../../../../dist/program'),
    'program-keypair.json'
);

async function main() {
    console.log("Launching client...");
    // Connect to Solana dev net
    let connection = new Connection('https://api.devnet.solana.com', 'confirmed');

    // Get program's public key
    const secretKeyString = fs.readFileSync(PROGRAM_KEYPAIR_PATH, {encoding: 'utf-8'});
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    const programKeypair = Keypair.fromSecretKey(secretKey);
    const programId: PublicKey = programKeypair.publicKey;

    // Generate an account to transact with program
    // const triggerKeypair = Keypair.generate();
    // const airdropRequest = await connection.requestAirdrop(
    //     triggerKeypair.publicKey,
    //     LAMPORTS_PER_SOL,
    // );
    // await connection.confirmTransaction(airdropRequest);

    const triggerSecretKeyString = fs.readFileSync("/home/ssuchichan/.config/solana/id.json", "utf-8");
    const triggerSecret = Uint8Array.from(JSON.parse(triggerSecretKeyString));
    const triggerKeypair = Keypair.fromSecretKey(triggerSecret);
    console.log(`Trigger: ${triggerKeypair.publicKey.toBase58()}`);

    // Conduct a transaction with program
    console.log('--Pinging Program', programId.toBase58());
    const instruction = new TransactionInstruction({
        keys: [{
            pubkey: triggerKeypair.publicKey, 
            isSigner: false, 
            isWritable: true
        }],
        programId: programId,
        data: Buffer.alloc(0)
    });

    const signature = await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [triggerKeypair]
    );
    console.log(`Transaction signature: ${signature}`)
}


main();

