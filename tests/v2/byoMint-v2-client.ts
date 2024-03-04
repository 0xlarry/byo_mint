import {Connection, LAMPORTS_PER_SOL, PublicKey, Transaction, sendAndConfirmTransaction, SYSVAR_INSTRUCTIONS_PUBKEY, TransactionMessage, VersionedTransaction, Keypair} from "@solana/web3.js";
import {TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { MPL_BUBBLEGUM_PROGRAM_ID, createSplNoopProgram, findTreeConfigPda } from "@metaplex-foundation/mpl-bubblegum";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { PublicKey as UmiPK } from "@metaplex-foundation/umi";
import * as anchor from "@coral-xyz/anchor";
import {PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata";
import { SPL_NOOP_PROGRAM_ID, SPL_ACCOUNT_COMPRESSION_PROGRAM_ID, createAllocTreeIx } from "@solana/spl-account-compression";
import { findByoMintPda } from "../byoMint-client";

const connection = new Connection(String(process.env.RPC));

// *********************************************************************
// PDAs
// *********************************************************************
export const findMetadataMapV2Pda = (program, authority, symbol) => {
    let [mdMapPda] = PublicKey.findProgramAddressSync([
        Buffer.from("v2"),
        authority.toBuffer(),
        Buffer.from(symbol)
      ], program.programId);
    return mdMapPda;
}

export const findFaucetV2Pda = (program, authority, metadataMap) => {
  let [faucetPda] = PublicKey.findProgramAddressSync([
    Buffer.from("v2"),
    authority.toBuffer(),
    metadataMap.toBuffer()
  ], program.programId);
  return faucetPda;
}

// *********************************
// METADATA MAP V2 IXs
// *********************************
export const createMetadataMapV2Ix = async (
    program: any,
    signerPubkey: PublicKey, 
    sellerFeeBasisPoints: number, 
    layers: any,
    uriPrefix: string,
    symbol: string,
    creators: any
) => {
    const metadataMap = findMetadataMapV2Pda(program, signerPubkey, symbol);
    return program.methods.createMetadataMapV2({
        sellerFeeBasisPoints,
        layers: Buffer.from(layers),
        uriPrefix,
        symbol,
        creators: creators
    }).accounts({
        auth: signerPubkey,
        metadataMap,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction();
}

export const deleteMetadataMapV2Ix = async (
    program: any,
    signerPubkey: PublicKey,
    metadataMap: PublicKey
) => {
    return program.methods.deleteMetadataMapV2().accounts({
        auth: signerPubkey,
        metadataMap: metadataMap,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction();
}

// *********************************
// FAUCET V2 IXs
// *********************************
export const createFaucetV2Ix = async (
    program: any,
    signerPubkey: PublicKey,
    metadataMapPda: PublicKey,
    collectionName: string,
    collectionSymbol: string,
    collectionUri: string,
    mintPrice: number,
    initialSupply: number
) => {
    const faucet = findFaucetV2Pda(program, signerPubkey, metadataMapPda);
    const collectionMint = anchor.web3.Keypair.generate();
    const associatedTokenAccount = anchor.utils.token.associatedAddress({
      mint: collectionMint.publicKey,
      owner: faucet,
    });
    console.log(`-- Collection Mint: ${collectionMint.publicKey}`);
    const metadataAddress = (await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        collectionMint.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    ))[0];
    console.log("-- Collection Metadata: ", metadataAddress.toBase58());
    const masterEditionAddress = (await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        collectionMint.publicKey.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID
    ))[0];
    console.log("-- Collection Master edition:", masterEditionAddress.toBase58());
    return {
        ix: await program.methods.createFaucetV2({
                collectionName,
                collectionSymbol,
                collectionUri,
                mintPrice: new anchor.BN(mintPrice),
                supplyCap: new anchor.BN(initialSupply)
            },).accounts({
                faucetAuth: signerPubkey,
                faucet,
                metadataMap: metadataMapPda,
                mint: collectionMint.publicKey,
                associatedTokenAccount,
                metadataAccount: metadataAddress,
                masterEditionAccount: masterEditionAddress,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY
            }).instruction(),
        collectionMint
    }
}

export const addNewTreeV2Ix = async (
    program: any,
    signerPubkey: PublicKey,
    faucetPda: PublicKey
) => {
    const emptyMerkleTree = anchor.web3.Keypair.generate();
    console.log(`Merke tree: ${emptyMerkleTree.publicKey.toBase58()}`);
    const umi = createUmi(process.env.RPC);
    const treeConfig = findTreeConfigPda(
      umi,
      {
        merkleTree: emptyMerkleTree.publicKey.toBase58() as UmiPK,
      }
    )[0]

    const treeConfigPublicKey = new anchor.web3.PublicKey(treeConfig)
    console.log('treeConfigPublicKey', treeConfigPublicKey.toBase58())

    // the tree space needs to be allocated in a separate non-nested instruction
    // as a nested CPI call cannot reallocate more than 10KB of space
    const allocTreeIx = await createAllocTreeIx(
      new Connection(process.env.RPC),
      emptyMerkleTree.publicKey,
      signerPubkey,
      { maxDepth: 14, maxBufferSize: 64 },
      11,
    );
    return {
        ix: await program.methods.addNewTreeV2().accounts({
                faucetAuth: signerPubkey,
                faucet: faucetPda,
                merkleTree: emptyMerkleTree.publicKey,
                treeConfig,
                bubblegumProgram: MPL_BUBBLEGUM_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                logWrapper: SPL_NOOP_PROGRAM_ID,
                compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID
            }).instruction(),
        emptyMerkleTree,
        allocTreeIx
    }
}

export const mintV2Ix = async (
    program: any,
    signerPubkey: PublicKey,
    faucetPda: PublicKey,
    layers: any,
    extraData = null
) => {
    const facuetAccount = await program.account.faucetV2.fetch(faucetPda);
    const metadataMap = await program.account.metadataMapV2.fetch(facuetAccount.metadataMap)
    const umi = createUmi(process.env.RPC);
    const [treeConfig] = findTreeConfigPda(umi,{merkleTree: facuetAccount.merkleTree});
    const [metadataAddress] = await anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        facuetAccount.collectionMint.toBuffer(),
    ], TOKEN_METADATA_PROGRAM_ID);
    const [masterEditionAddress] = await anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        facuetAccount.collectionMint.toBuffer(),
        Buffer.from("edition"),
    ], TOKEN_METADATA_PROGRAM_ID);
    const [bubblegumSigner] = PublicKey.findProgramAddressSync(
        [Buffer.from("collection_cpi", "utf8")],
        new anchor.web3.PublicKey(MPL_BUBBLEGUM_PROGRAM_ID)
    );

    return await program.methods.mintV2({
        layers: Buffer.from(layers),
        extraData
    }).accounts({
        minter: signerPubkey,
        faucet: faucetPda,
        metadataMap: facuetAccount.metadataMap,
        byoMint: findByoMintPda(program, facuetAccount.metadataMap, layers),
        treeConfig: new PublicKey(treeConfig),
        leafOwner: signerPubkey,
        merkleTree: facuetAccount.merkleTree,
        collectionMint: facuetAccount.collectionMint,
        collectionMetadata: metadataAddress,
        editionAccount: masterEditionAddress,
        bubblegumSigner,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        bubblegumProgram: new PublicKey(MPL_BUBBLEGUM_PROGRAM_ID),
        systemProgram: anchor.web3.SystemProgram.programId,
        creator: metadataMap.creators[0].address
      }).instruction();
}

export const updateFaucetV2Ix = async (
    program: any,
    signerPubkey: PublicKey,
    faucetPda: PublicKey,
    newSupply: number | null,
    newMintPrice: number | null,
) => {
    return await program.methods.updateFaucetV2({
        supplyCap: newSupply ? new anchor.BN(newSupply) : null,
        mintPrice: newMintPrice ? new anchor.BN(newMintPrice) : null,
    }).accounts({
        faucetAuth: signerPubkey,
        faucet: faucetPda,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction()
}