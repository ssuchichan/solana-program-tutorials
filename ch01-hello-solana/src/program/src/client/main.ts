import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
  } from '@solana/web3.js';
import fs from 'mz/fs';
import path from 'path';

const PROGRAM_KEYPAIR_PATH = path.join(
    path.resolve(__dirname, '../../dist/program'),
    'program-keypair.json'
);

async function main() {
    console.log("Launching client...");
    // Connect to Solana dev net
    let connection = new Connection('https://api.devnet.solana.com', 'confirmed');

    const secretKeyString = await fs.readFile(PROGRAM_KEYPAIR_PATH, {encoding: 'uft-8'});
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    const programKeypair = Keypair.fromSecretKey(secretKey);
    let programID: PublicKey = programKeypair.publicKey;
}






