import {Connection, LAMPORTS_PER_SOL, PublicKey, Transaction, sendAndConfirmTransaction, SYSVAR_INSTRUCTIONS_PUBKEY, TransactionMessage, VersionedTransaction} from "@solana/web3.js";
import {TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { MPL_BUBBLEGUM_PROGRAM_ID, createSplNoopProgram, findTreeConfigPda } from "@metaplex-foundation/mpl-bubblegum";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { PublicKey as UmiPK } from "@metaplex-foundation/umi";
import * as anchor from "@coral-xyz/anchor";
import {PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata";
import { SPL_NOOP_PROGRAM_ID, SPL_ACCOUNT_COMPRESSION_PROGRAM_ID, createAllocTreeIx } from "@solana/spl-account-compression";

const connection = new Connection(String(process.env.RPC));
// *********************************************************************
// PDAs
// *********************************************************************
export const findLayerMapPda = (program, authority, symbol) => {
    let [layerMapPda] = PublicKey.findProgramAddressSync([
        anchor.utils.bytes.utf8.encode("layer"),
        authority.toBuffer(),
        Buffer.from(symbol)
      ], program.programId);
    return layerMapPda;
}

export const findSupplyMapPda = (program, authority, symbol) => {
    let [supplyMapPda] = PublicKey.findProgramAddressSync([
        anchor.utils.bytes.utf8.encode("supply"),
        authority.toBuffer(),
        Buffer.from(symbol)
      ], program.programId);
    return supplyMapPda;
}

export const findFaucetV2Pda = (program, authority, collectionMint) => {
    let [faucetV2Pda] = PublicKey.findProgramAddressSync([
      authority.toBuffer(),
      collectionMint.toBuffer()
    ], program.programId);
    return faucetV2Pda;
}

export const findByoNftPda = (program, metadataMapPda, mintIndex) => {
    const numberBuffer = Buffer.alloc(8);
    const indexSeed = numberBuffer.writeBigInt64LE(mintIndex, 0);
    let [byoNftPda] = PublicKey.findProgramAddressSync([
        metadataMapPda.toBuffer(),
        indexSeed,
      ], program.programId);
    return byoNftPda;
}

// TODO TRAIT_COMBO_PDA

// *********************************
// METADATA MAPs
// *********************************
export const createSupplyMapIx = async (
    program: any,
    signerPubkey: PublicKey, 
    sellerFeeBasisPoints: number, 
    uriPrefix: string,
    symbol: string,
    creators: any, 
    items: any
) => {
    const supplyMap = findSupplyMapPda(program, signerPubkey, symbol);
    return program.methods.createSupplyMax({
        sellerFeeBasisPoints,
        uriPrefix,
        symbol,
        creators,
        items
    }).accounts({
        signer: signerPubkey,
        supplyMap,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction();
}

// TODO CREATE LAYER MAP


// *********************************
// FAUCET V2
// *********************************
export const createFaucetV2Ix = async (
    program: any,
    signerPubkey: PublicKey,
    layerMap: PublicKey | null,
    supplyMap: PublicKey | null,
    collectionName: string,
    collectionSymbol: string,
    collectionUri: string,
    supplyCap: null,
    mintPrice: number,
    mintToken: PublicKey | null,
    wlCollection: PublicKey | null,
) => {
    const collectionMint: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    const faucet = findFaucetV2Pda(program, signerPubkey, collectionMint.publicKey);
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
                supplyCap: new anchor.BN(supplyCap),
                mintToken,
                wlCollection
            },).accounts({
                faucetAuth: signerPubkey,
                faucet,
                layerMap,
                supplyMap,
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

export const addNewTreeV2 = async (
    program: any,
    signerPubkey: PublicKey,
    faucetV2Pda: PublicKey
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
                faucet: faucetV2Pda,
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

export const updateFaucetV2Ix = async (
    program: any,
    signerPubkey: PublicKey,
    faucetV2Pda: PublicKey,
    supplyMap: PublicKey | null,
    layerMap: PublicKey | null,
    supplyCap: Number | null,
    mintPrice: Number | null
) => {
    return await program.methods.updateFaucetV2({
        supplyCap,
        mintPrice
    }).accounts({
        faucetAuth: signerPubkey,
        faucet: faucetV2Pda,
        supplyMap,
        layerMap,
        systemProgram: anchor.web3.SystemProgram.programId,
    }).instruction()
}

export const mintSupplyMapIx = async (
    program: any,
    signerPubkey: PublicKey,
    faucetV2Pda: PublicKey,
    supplyMap: PublicKey,
) => {
    const facuetAccount = await program.account.faucetV2.fetch(faucetV2Pda);
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
    return await program.methods.mintSupplyMap().accounts({
        minter: signerPubkey,
        faucet: faucetV2Pda,
        supplyMap,
        treeConfig,
        leafOwner: signerPubkey,
        collectionMint: facuetAccount.collectionMint,
        collectionMetadata: metadataAddress,
        editionAccount: masterEditionAddress,
        bubblegumSigner,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        bubblegumProgram: new PublicKey(MPL_BUBBLEGUM_PROGRAM_ID),
        systemProgram: anchor.web3.SystemProgram.programId,
    }).instruction();
}
