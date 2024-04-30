import dotenv from "dotenv";
import { Connection } from "@solana/web3.js";

//Utility function to check the status of a transaction, return bad execution if a transaction is dropped.
export async function checkTxStatus(txId: string) {
    dotenv.config();

    const SOLANA_RPC = new Connection(process.env.RPC_URL);
    const start = new Date();

    const blockhashResponse = await SOLANA_RPC.getLatestBlockhashAndContext('finalized');
    const lastValidHeight = blockhashResponse.value.lastValidBlockHeight;
    
    let hashExpired = false;
    let txStatus = false;
    while (!txStatus) {
        const { value: status } = await SOLANA_RPC.getSignatureStatus(txId);
        if (status && ((status.confirmationStatus === 'confirmed' || status.confirmationStatus === 'finalized'))) {
            txStatus = true;
            const endTime = new Date();
            const elapsed = (endTime.getTime() - start.getTime())/1000;
            console.log(`Transaction Success. Elapsed time: ${elapsed} seconds.`);
            console.log(`https://explorer.solana.com/tx/${txId}?cluster=devnet`);
            break;
        }

        hashExpired = await isBlockhashExpired(SOLANA_RPC, lastValidHeight);

        // Break loop if blockhash has expired
        if (hashExpired) {
            const endTime = new Date();
            const elapsed = (endTime.getTime() - start.getTime())/1000;
            console.log(`Blockhash has expired. Elapsed time: ${elapsed} seconds.`);
            console.log(`Try to send a new transaction`);
            break;
        }
        //Wait 1 second to re-check status
        new Promise(resolve => setTimeout(resolve, 1000));
    }
}

async function isBlockhashExpired(connection: Connection, lastValidBlockHeight: number) {
    let currentBlockHeight = (await connection.getBlockHeight('finalized'));
    console.log('                           ');
    console.log('Current Block height:             ', currentBlockHeight);
    console.log('Last Valid Block height - 150:     ', lastValidBlockHeight - 150);
    console.log('--------------------------------------------');    
    console.log('Difference:                      ',currentBlockHeight - (lastValidBlockHeight-150)); // If Difference is positive, blockhash has expired.
    console.log('                           ');

    return (currentBlockHeight > lastValidBlockHeight - 150);
}