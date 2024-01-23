import {ComputeBudgetProgram, Connection, Keypair, PublicKey, SYSVAR_INSTRUCTIONS_PUBKEY, Transaction, sendAndConfirmTransaction} from "@solana/web3.js";
import {TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { MPL_BUBBLEGUM_PROGRAM_ID, findTreeConfigPda } from "@metaplex-foundation/mpl-bubblegum";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { PublicKey as UmiPK } from "@metaplex-foundation/umi";
import * as anchor from "@coral-xyz/anchor";
import {PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata";
import { SPL_NOOP_PROGRAM_ID, SPL_ACCOUNT_COMPRESSION_PROGRAM_ID, createAllocTreeIx } from "@solana/spl-account-compression";
import axios from "axios";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import fs from 'fs';


const connection = new Connection(String(process.env.RPC));
export const loadCliWallet = (filepath) => {
    const data = fs.readFileSync(filepath);
    return Keypair.fromSecretKey(new Uint8Array(JSON.parse(data.toString())));
}
// *********************************
// EXECUTE TX
// *********************************
export const executeTx = async (keypair, ixs, extraSigner = null, finalized = false, skipPreflight = false) => {
    const tx = new Transaction();
    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({ 
        units: 1000000 
    });
    tx.add(modifyComputeUnits);
    ixs.forEach(ix => tx.add(ix) );
    const { blockhash } = await connection.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.feePayer = keypair.publicKey;
    const signers = [keypair];
    if (extraSigner) {
        signers.push(extraSigner);
    }
    console.log("++ ABOUT TO SIGN");
    const sig = await sendAndConfirmTransaction(connection, tx, signers, {
        commitment: finalized ? 'finalized' : 'confirmed',
        skipPreflight
    });
    console.log({sig});
    return sig;
}

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
export const findOpenMapPda = (program, authority, symbol) => {
    let [openMapPda] = PublicKey.findProgramAddressSync([
        anchor.utils.bytes.utf8.encode("open"),
        authority.toBuffer(),
        Buffer.from(symbol)
      ], program.programId);
    return openMapPda;
}
export const findFaucetV2Pda = (program, authority, collectionMint) => {
    let [faucetV2Pda] = PublicKey.findProgramAddressSync([
      authority.toBuffer(),
      collectionMint.toBuffer()
    ], program.programId);
    return faucetV2Pda;
}
export const findTraitComboPda = (program, layerMap, layers) => {
    let [traitComboPda] = PublicKey.findProgramAddressSync([
      layerMap.toBuffer(),
      Buffer.from(layers),
    ], program.programId);
    return traitComboPda;
}
export const findBackgroundPda = (program, mint) => {
    let [bgPda] = PublicKey.findProgramAddressSync([
        anchor.utils.bytes.utf8.encode("bg"),
        mint.toBuffer()
    ], program.programId);
    return bgPda;
}

// *********************************
// METADATA MAPs
// *********************************
export const createLayerMapIx = async (
    program: any,
    signerPubkey: PublicKey, 
    sellerFeeBasisPoints: number, 
    uriPrefix: string,
    symbol: string,
    creators: any, 
    layers: any
) => {
    const layerMap = findLayerMapPda(program, signerPubkey, symbol);
    return program.methods.createLayerMap({
        sellerFeeBasisPoints,
        uriPrefix,
        symbol,
        creators,
        layers
    }).accounts({
        auth: signerPubkey,
        layerMap,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction();
}
export const createSupplyMapIx = async (
    program: any,
    signerPubkey: PublicKey, 
    sellerFeeBasisPoints: number, 
    symbol: string,
    creators: any, 
    items: any
) => {
    const supplyMap = findSupplyMapPda(program, signerPubkey, symbol);
    return program.methods.createSupplyMap({
        sellerFeeBasisPoints,
        symbol,
        creators,
        items
    }).accounts({
        signer: signerPubkey,
        supplyMap,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction();
}
export const createOpenMapIx = async (
    program: any,
    signerPubkey: PublicKey, 
    sellerFeeBasisPoints: number, 
    symbol: string,
    creators: any, 
) => {
    const openMap = findOpenMapPda(program, signerPubkey, symbol);
    return program.methods.createOpenMap({
        sellerFeeBasisPoints,
        symbol,
        creators
    }).accounts({
        signer: signerPubkey,
        openMap,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction();
}


// *********************************
// FAUCET V2
// *********************************
export const createFaucetV2Ix = async (
    program: any,
    signerPubkey: PublicKey,
    layerMap: PublicKey | null,
    supplyMap: PublicKey | null,
    openMap: PublicKey | null,
    collectionName: string,
    collectionSymbol: string,
    collectionUri: string,
    supplyCap: number,
    mintPrice: number,
    mintToken: PublicKey | null,
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
                mintToken
            },).accounts({
                faucetAuth: signerPubkey,
                faucet,
                layerMap,
                supplyMap,
                openMap,
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
    layerMap: PublicKey | null,
    supplyMap: PublicKey | null,
    openMap: PublicKey | null,
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
        openMap,
        systemProgram: anchor.web3.SystemProgram.programId,
    }).instruction()
}

// *********************************
// MINT
// *********************************
const getCollectionAccounts = async (faucetAccount) => {
    const [metadataAddress] = await anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        faucetAccount.collectionMint.toBuffer(),
    ], TOKEN_METADATA_PROGRAM_ID);
    const [masterEditionAddress] = await anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        faucetAccount.collectionMint.toBuffer(),
        Buffer.from("edition"),
    ], TOKEN_METADATA_PROGRAM_ID);
    const [bubblegumSigner] = PublicKey.findProgramAddressSync(
        [Buffer.from("collection_cpi", "utf8")],
        new anchor.web3.PublicKey(MPL_BUBBLEGUM_PROGRAM_ID)
    );
    return {metadataAddress, masterEditionAddress, bubblegumSigner};
}
const getFeeAccounts = async (faucetAccount, mapAccount, signerPubkey) => {
    const creator = mapAccount.creators[0].address;
    let creatorTa = null, minterTa = null;
    if (faucetAccount.mintToken.toString() !== PublicKey.default.toString()) {
        creatorTa = await getAssociatedTokenAddress(faucetAccount.mintToken, creator);
        minterTa = await getAssociatedTokenAddress(faucetAccount.mintToken, signerPubkey);
    }
    return {creator, creatorTa, minterTa};
}
export const mintLayerMapIx = async (
    program: any,
    signerPubkey: PublicKey,
    faucetV2Pda: PublicKey,
    layers: any,
    bgColor: any
) => {
    // faucet accounts
    const faucetAccount = await program.account.faucetV2.fetch(faucetV2Pda);
    const layerMap = await program.account.layerMap.fetch(faucetAccount.layerMap);
    const umi = createUmi(process.env.RPC);
    const [treeConfig] = findTreeConfigPda(umi,{merkleTree: faucetAccount.merkleTree});
    // collection accounts
    const {metadataAddress, masterEditionAddress, bubblegumSigner} = await getCollectionAccounts(faucetAccount);
    // fee accounts
    const {creator, creatorTa, minterTa} = await getFeeAccounts(faucetAccount, layerMap, signerPubkey);
    return await program.methods.mintLayerMap({
        layers: Buffer.from(layers),
        bgColor
    }).accounts({
        minter: signerPubkey,
        faucet: faucetV2Pda,
        layerMap: faucetAccount.layerMap,
        traitCombo: findTraitComboPda(program, faucetAccount.layerMap, layers),
        treeConfig,
        merkleTree: faucetAccount.merkleTree,
        leafOwner: signerPubkey,
        collectionMint: faucetAccount.collectionMint,
        collectionMetadata: metadataAddress,
        editionAccount: masterEditionAddress,
        bubblegumSigner,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        bubblegumProgram: new PublicKey(MPL_BUBBLEGUM_PROGRAM_ID),
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        creator, // TODO CHECK
        creatorTa,
        minterTa
    }).instruction();
}
export const mintSupplyMapIx = async (
    program: any,
    signerPubkey: PublicKey,
    faucetV2Pda: PublicKey,
) => {
    // faucet accounts
    const faucetAccount = await program.account.faucetV2.fetch(faucetV2Pda);
    const umi = createUmi(process.env.RPC);
    const [treeConfig] = findTreeConfigPda(umi,{merkleTree: faucetAccount.merkleTree});
    const supplyMap = await program.account.supplyMap.fetch(faucetAccount.supplyMap);
    // collection accounts
    const {metadataAddress, masterEditionAddress, bubblegumSigner} = await getCollectionAccounts(faucetAccount);
    // fee accounts
    const {creator, creatorTa, minterTa} = await getFeeAccounts(faucetAccount, supplyMap, signerPubkey);
    return await program.methods.mintSupplyMap().accounts({
        minter: signerPubkey,
        faucet: faucetV2Pda,
        supplyMap,
        treeConfig,
        leafOwner: signerPubkey,
        collectionMint: faucetAccount.collectionMint,
        collectionMetadata: metadataAddress,
        editionAccount: masterEditionAddress,
        bubblegumSigner,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        bubblegumProgram: new PublicKey(MPL_BUBBLEGUM_PROGRAM_ID),
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        creator, // TODO CHECK
        creatorTa,
        minterTa
    }).instruction();
}
export const mintOpenMapIx = async (
    program: any,
    signerPubkey: PublicKey,
    faucetV2Pda: PublicKey,
    name: String, 
    jsonUri: String
) => {
    // faucet accounts
    const faucetAccount = await program.account.faucetV2.fetch(faucetV2Pda);
    const umi = createUmi(process.env.RPC);
    const [treeConfig] = findTreeConfigPda(umi,{merkleTree: faucetAccount.merkleTree});
    const openMap = await program.account.openMap.fetch(faucetAccount.openMap);
    // collection accounts
    const {metadataAddress, masterEditionAddress, bubblegumSigner} = await getCollectionAccounts(faucetAccount);
    // fee accounts
    const {creator, creatorTa, minterTa} = await getFeeAccounts(faucetAccount, openMap, signerPubkey);
    return await program.methods.mintOpenMap({
        name,
        jsonUri
    }).accounts({
        minter: signerPubkey,
        faucet: faucetV2Pda,
        openMap,
        treeConfig,
        merkleTree: faucetAccount.merkleTree,
        leafOwner: signerPubkey,
        collectionMint: faucetAccount.collectionMint,
        collectionMetadata: metadataAddress,
        editionAccount: masterEditionAddress,
        bubblegumSigner,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        bubblegumProgram: new PublicKey(MPL_BUBBLEGUM_PROGRAM_ID),
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        creator, // TODO CHECK
        creatorTa,
        minterTa
    }).instruction();
}

// *********************************
// BACKGROUND IXs
// *********************************
export const closeBackgroundIx = async (
    program: any,
    signerPubkey: PublicKey,
    mint: PublicKey
) => {
    const {proofPathAsAccounts, params, merkleTree} = await getCnftAccounts(mint);
    const bgPda = findBackgroundPda(program, mint);
    return await program.methods.closeBg(params).accounts({
        leafOwner: signerPubkey,
        background: bgPda,
        merkleTree: merkleTree,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
    })
    .remainingAccounts(proofPathAsAccounts)
    .instruction();
}
export const setBgColorIx = async (
    program: any,
    signerPubkey: PublicKey,
    mint: PublicKey,
    bgColor: String,
) => {
    const {proofPathAsAccounts, params, merkleTree} = await getCnftAccounts(mint);
    const bgPda = findBackgroundPda(program, mint);
    params['bgColor'] = bgColor;
    console.log({proofPathAsAccounts});
    return await program.methods.setBgColor(params).accounts({
        leafOwner: signerPubkey,
        background: bgPda,
        merkleTree: merkleTree,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId
    })
    .remainingAccounts(proofPathAsAccounts)
    .instruction();
}
// TODO: add bg asset
// TODO: remove bg asset

// *********************************
// UTILs
// *********************************
export const getCnftAccounts = async (assetId, loadingLoot = false) => {
    // fetch asset data
    const asset = await getAsset(assetId);
    const proof = await getAssetProof(assetId);

    // accounts
    const leafDelegate = asset.ownership.delegate ? new PublicKey(asset.ownership.delegate) : new PublicKey(asset.ownership.owner);
    const proofPathAsAccounts = mapProof(proof);
    console.log("PROOF LENGTH = ", proofPathAsAccounts.length);
    console.log("TREE PUBKEY = ", asset.compression.tree)

    // params
    const root = decode(proof.root);
    const dataHash = decode(asset.compression.data_hash);
    const creatorHash = decode(asset.compression.creator_hash);
    const nonce = new anchor.BN(asset.compression.leaf_id);
    const index = asset.compression.leaf_id;
    const retObj = {
        leafDelegate, 
        proofPathAsAccounts,
        merkleTree: new PublicKey(asset.compression.tree),
        params: {
            root,
            dataHash,
            creatorHash,
            nonce,
            index
        }
    }
    if (!loadingLoot) {
        retObj.params['name'] = asset.content.metadata.name;
        retObj.params['uri'] = asset.content.json_uri;
        retObj.params['symbol'] = asset.content.metadata.symbol;
    }
    return retObj;
}
export async function getAsset(assetId: any, rpcUrl = process.env.RPC): Promise<any> {
    try {
        const axiosInstance = axios.create({
            baseURL: rpcUrl,
        });
        const response = await axiosInstance.post(rpcUrl, {
            jsonrpc: "2.0",
            method: "getAsset",
            id: "rpd-op-123",
            params: {
            id: assetId
            },
        });
        return response.data.result;
    } catch (error) {
        console.error(error);
    }
}
export async function getAssetProof(assetId: any, rpcUrl = process.env.RPC): Promise<any> {
    try {
        const axiosInstance = axios.create({
            baseURL: rpcUrl,
        });
        const response = await axiosInstance.post(rpcUrl, {
                jsonrpc: "2.0",
                method: "getAssetProof",
                id: "rpd-op-123",
                params: {
                id: assetId
            },
        });
        return response.data.result;
    } catch (error) {
        console.error(error);
    }
}
export function decode(stuff: string) {
    return bufferToArray(bs58.decode(stuff))
}
function bufferToArray(buffer: Buffer): number[] {
    const nums: number[] = [];
    for (let i = 0; i < buffer.length; i++) {
        nums.push(buffer[i]);
    }
    return nums;
}
export const mapProof = (assetProof: any) => {
    if (!assetProof.proof || assetProof.proof.length === 0) {
        throw new Error("Proof is empty");
    }
    return assetProof.proof.map((node:any) => ({
        pubkey: new PublicKey(node),
        isSigner: false,
        isWritable: false,
    }));
};