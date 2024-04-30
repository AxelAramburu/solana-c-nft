import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import dotenv from "dotenv";
import { SolanaCNft } from "../target/types/solana_c_nft";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { findMasterEditionPda, findMetadataPda, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { publicKey } from "@metaplex-foundation/umi";
import { toWeb3JsKeypair, toWeb3JsPublicKey } from "@metaplex-foundation/umi-web3js-adapters";
import { ValidDepthSizePair, createAllocTreeIx } from "@solana/spl-account-compression";
import { checkTxStatus } from "../app/utils";

describe("solana-c-nft", async () => {
  dotenv.config();
  const endpoint = process.env.RPC_URL;
  const umi = createUmi(endpoint).use(mplTokenMetadata());

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);  
  
  const program = anchor.workspace.SolanaCNft as Program<SolanaCNft>;
  const seeds = [Buffer.from("RSTSeed")];
  console.log("Program: ", program.programId.toBase58());
  
  let signer = provider.wallet as anchor.Wallet;
  console.log("Signer: ", signer.publicKey.toBase58());

  const [collectionPda] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
  console.log("Collection PDA: ", collectionPda.toBase58());
  // Get the associated token address(ATA)
  const ata = await getAssociatedTokenAddress(collectionPda, signer.publicKey);
  console.log("ATA Pubkey: ", ata.toBase58());

  const mplTokenProgram = publicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  const bubblegumProgram = publicKey("BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY");
  const compressionProgram = publicKey("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK");
  const noopProgram = publicKey("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV");
  // Derive the metadata account
  let metadataAccount = await findMetadataPda(umi, {
    mint: publicKey(collectionPda),
  })[0];
  console.log("Metadata account: ", metadataAccount);

  // Derive the master edition pda
  let masterEditionAccount = await findMasterEditionPda(umi, {
    mint: publicKey(collectionPda),
  })[0];
  console.log("Master Edition account: ", masterEditionAccount);

  //Trees accounts
  let merkleTree = umi.eddsa.generateKeypair();
  console.log("Merkle Tree: ", merkleTree.publicKey);

  let [merkleTreeAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
    [toWeb3JsPublicKey(merkleTree.publicKey).toBuffer()], toWeb3JsPublicKey(bubblegumProgram),
  );
  console.log("Merkle Tree Auth: ", merkleTreeAuthority.toBase58());
  
  const metadata = {
		name: "RandomSolanaToken",
		symbol: "RST",
    uri: "https://raw.githubusercontent.com/AxelAramburu/solana-c-nft/dev/uri.json"
  };

  it("Initialize cNFT!", async () => {
    try {
      const initTx = await program.methods.initCnft(metadata.name, metadata.symbol, metadata.uri)
        .accounts({
          signer: signer.publicKey,
          collectionMint: collectionPda,
          metadataAccount: metadataAccount,
          masterEditionAccount: masterEditionAccount,
          treeAuthority: publicKey(merkleTreeAuthority),
          merkleTree: merkleTree.publicKey,
          tokenAccount: ata,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenMetadataProgram: mplTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          bubblegumProgram: bubblegumProgram,
          compressionProgram: compressionProgram,
        })
        .signers([signer.payer])
        .rpc();
  
      checkTxStatus(initTx);
      console.log("Init CNft Tx: ", initTx);

    } catch (error) {
      console.log(error);
    }
  })

  it("Initialize Tree", async () => {
    // On your project, warning to specify theses parameters based on your needs
    const maxDepthSizePair: ValidDepthSizePair = {
      maxDepth: 14,
      maxBufferSize: 64,
    };
    console.log("Max Depth computed: ", maxDepthSizePair.maxDepth);
    // More details on canopyDepth here: https://solana.com/docs/advanced/state-compression#sizing-a-concurrent-merkle-tree
    const canopyDepth = 4;

    try {
      // instruction to create new account with required space for tree
      const allocTreeIx = await createAllocTreeIx(
        provider.connection,
        toWeb3JsPublicKey(merkleTree.publicKey),
        signer.publicKey,
        maxDepthSizePair,
        canopyDepth
      );

      const createTreeTx = new anchor.web3.Transaction().add(allocTreeIx)

      const txSignature = await anchor.web3.sendAndConfirmTransaction(
        provider.connection,
        createTreeTx,
        [signer.payer, toWeb3JsKeypair(merkleTree)],
        {
          commitment: "confirmed",
        }
      )
      console.log("Creation of the tree tx: ", txSignature);

      const treeTx = await program.methods.initTree()
      .accounts({
        signer: signer.publicKey,
        collectionMint: collectionPda,
        treeAuthority: publicKey(merkleTreeAuthority),
        merkleTree: merkleTree.publicKey,
        logWrapper: noopProgram,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        bubblegumProgram: bubblegumProgram,
        compressionProgram: compressionProgram,
      })
      .signers([signer.payer])
      .rpc();

      checkTxStatus(treeTx);
      console.log("Tree Tx: ", treeTx);

    } catch (error) {
      console.log(error);
    }
  })

  it("Mint a cNft token !", async () => {
    const bubblegumSigner = umi.eddsa.findPda(bubblegumProgram, seeds);
    
    try {
      const mintTx = await program.methods.mintCnft()
      .accounts({
        signer: signer.publicKey,
        collectionMint: collectionPda,
        treeAuthority: publicKey(merkleTreeAuthority),
        merkleTree: merkleTree.publicKey,
        bubblegumSigner: publicKey(bubblegumSigner),
        logWrapper: noopProgram,
        tokenMetadataProgram: mplTokenProgram,
        systemProgram: anchor.web3.SystemProgram.programId,
        collectionMetadata: metadataAccount,
        editionAccount: masterEditionAccount,
        bubblegumProgram: bubblegumProgram,
        compressionProgram: compressionProgram,
      })
      .signers([signer.payer])
      .rpc();
  
      checkTxStatus(mintTx);
      console.log("Mint tx: ", mintTx);

    } catch (error) {
      console.log(error);
    }
  })
});
