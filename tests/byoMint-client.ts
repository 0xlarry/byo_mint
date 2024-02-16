import {Connection, LAMPORTS_PER_SOL, PublicKey, Transaction, sendAndConfirmTransaction, SYSVAR_INSTRUCTIONS_PUBKEY, TransactionMessage, VersionedTransaction} from "@solana/web3.js";
import {TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { MPL_BUBBLEGUM_PROGRAM_ID, createSplNoopProgram, findTreeConfigPda } from "@metaplex-foundation/mpl-bubblegum";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { PublicKey as UmiPK } from "@metaplex-foundation/umi";
import * as anchor from "@coral-xyz/anchor";
import {PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID} from "@metaplex-foundation/mpl-token-metadata";
import { SPL_NOOP_PROGRAM_ID, SPL_ACCOUNT_COMPRESSION_PROGRAM_ID, createAllocTreeIx } from "@solana/spl-account-compression";
import axios from "axios";
import bs58 from 'bs58';

const connection = new Connection(String(process.env.RPC));
// *********************************************************************
// PDAs
// *********************************************************************
export const findMetadataMapPda = (program, authority, symbol) => {
    let [mdMapPda] = PublicKey.findProgramAddressSync([
        authority.toBuffer(),
        Buffer.from(symbol)
      ], program.programId);
    return mdMapPda;
}

export const findByoMintPda = (program, metadataMapPda, traits) => {
    let [byoPda] = PublicKey.findProgramAddressSync([
        metadataMapPda.toBuffer(),
        Buffer.from(traits),
      ], program.programId);
    console.log("-- BYO PDA = ", byoPda.toString());
    return byoPda;
}

export const findFaucetPda = (program, authority, metadataMap) => {
  let [faucetPda] = PublicKey.findProgramAddressSync([
    authority.toBuffer(),
    metadataMap.toBuffer()
  ], program.programId);
  return faucetPda;
}

export const findFaucetWlPda = (program, metadataMap, mint) => {
    let [faucetWlPda] = PublicKey.findProgramAddressSync([
        anchor.utils.bytes.utf8.encode("wl"),
        metadataMap.toBuffer(),
        mint.toBuffer()
    ], program.programId);
    return faucetWlPda;
}

export const findWlMintPda = (program, metadataMap, mint) => {
    let [wlMintPda] = PublicKey.findProgramAddressSync([
        metadataMap.toBuffer(),
        mint.toBuffer()
    ], program.programId);
    return wlMintPda;
}

export const findBackgroundPda = (program, mint) => {
    let [bgPda] = PublicKey.findProgramAddressSync([
        anchor.utils.bytes.utf8.encode("bg"),
        mint.toBuffer()
    ], program.programId);
    return bgPda;
}

// *********************************
// EXECUTE TX
// *********************************
export const executeTx = async (keypair, ixs, extraSigner = null, finalized = false, skipPreflight = false) => {
    const tx = new Transaction();
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

// *********************************
// METADATA MAP IXs
// *********************************
export const createMetadataMapIx = async (
    program: any,
    signerPubkey: PublicKey, 
    sellerFeeBasisPoints: number, 
    layers: any,
    uriPrefix: string,
    symbol: string
) => {
    const metadataMap = findMetadataMapPda(program, signerPubkey, symbol);
    return program.methods.createMetadataMap({
        sellerFeeBasisPoints,
        layers: Buffer.from(layers),
        uriPrefix,
        symbol
    }).accounts({
        auth: signerPubkey,
        metadataMap,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction();
}

export const deleteMetadataMapIx = async (
    program: any,
    signerPubkey: PublicKey,
    metadataMap: PublicKey
) => {
    return program.methods.deleteMetadataMap().accounts({
        auth: signerPubkey,
        metadataMap: metadataMap,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction();
}

// *********************************
// FAUCET IXs
// *********************************
export const createFaucetIx = async (
    program: any,
    signerPubkey: PublicKey,
    metadataMapPda: PublicKey,
    collectionName: string,
    collectionSymbol: string,
    collectionUri: string,
    mintPrice: number
) => {
    const faucet = findFaucetPda(program, signerPubkey, metadataMapPda);
    const collectionMint: anchor.web3.Keypair = anchor.web3.Keypair.generate();
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
        ix: await program.methods.createFaucet({
                collectionName,
                collectionSymbol,
                collectionUri,
                mintPrice: new anchor.BN(mintPrice),
                supplyCap: new anchor.BN(100)
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

export const addNewTree = async (
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
        ix: await program.methods.addNewTree().accounts({
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

export const mintCnftIx = async (
    program: any,
    signerPubkey: PublicKey,
    faucetPda: PublicKey,
    layers: any,
    bgColor = null
) => {
    const facuetAccount = await program.account.faucet.fetch(faucetPda);
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

    return await program.methods.mint({
        layers: Buffer.from(layers),
        bgColor
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
      }).instruction();
}

export const updateFaucetIx = async (
    program: any,
    signerPubkey: PublicKey,
    faucetPda: PublicKey,
    newSupply: number | null,
    newMintPrice: number | null,
) => {
    return await program.methods.updateFaucet({
        supplyCap: newSupply ? new anchor.BN(newSupply) : null,
        mintPrice: newMintPrice ? new anchor.BN(newMintPrice) : null,
    }).accounts({
        faucetAuth: signerPubkey,
        faucet: faucetPda,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction()
}

export const payoutIx = async (
    program: any,
    signerPubkey: PublicKey,
    faucet: PublicKey
) => {
    return await program.methods.withdrawFees().accounts({
        auth: signerPubkey,
        faucet,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction()
}

// WL
export const creatWlFaucetIx = async (
    program: any,
    signerPubkey: PublicKey,
    metadataMapPda: PublicKey,
    collectionName: string,
    collectionSymbol: string,
    collectionUri: string,
    mintPrice: number,
    wlCollection: PublicKey
) => {
    const faucet = findFaucetWlPda(program, signerPubkey, metadataMapPda);
    const collectionMint: anchor.web3.Keypair = anchor.web3.Keypair.generate();
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
        ix: await program.methods.createFaucetWl({
                collectionName,
                collectionSymbol,
                collectionUri,
                mintPrice: new anchor.BN(mintPrice),
                supplyCap: new anchor.BN(10000),
                wlCollection
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

export const mintWlIx = async (
    program: any,
    signerPubkey: PublicKey,
    faucetPda: PublicKey,
    wlTokenMint: PublicKey,
    layers: any,
    bgColor = null
) => {
    const facuetAccount = await program.account.faucetWl.fetch(faucetPda);
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
    
    // wl mint accounts
    const wlMint = findWlMintPda(program, facuetAccount.metadataMap, wlTokenMint);
    const tokenAccount = await getAssociatedTokenAddress(wlTokenMint, signerPubkey);
    const [metadata] = await anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        wlTokenMint.toBuffer(),
    ], TOKEN_METADATA_PROGRAM_ID);

    return await program.methods.mintWl({
        layers: Buffer.from(layers),
        bgColor
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
        wlMint,
        tokenAccount,
        metadata
      }).instruction();
}

export const addNewTreeWl = async (
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
        ix: await program.methods.addNewTreeWl().accounts({
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

export const updateFaucetWlIx = async (
    program: any,
    signerPubkey: PublicKey,
    faucetPda: PublicKey,
    newSupply: number | null,
    newMintPrice: number | null,
) => {
    return await program.methods.updateFaucetWl({
        supplyCap: newSupply ? new anchor.BN(newSupply) : null,
        mintPrice: newMintPrice ? new anchor.BN(newMintPrice) : null,
    }).accounts({
        faucetAuth: signerPubkey,
        faucet: faucetPda,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction()
}

export const payoutWlIx = async (
    program: any,
    signerPubkey: PublicKey,
    faucet: PublicKey
) => {
    return await program.methods.withdrawFeesWl().accounts({
        faucetAuth: signerPubkey,
        faucet,
        systemProgram: anchor.web3.SystemProgram.programId
    }).instruction()
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



// utils
export const getCnftAccounts = async (assetId, loadingLoot = false) => {
    // fetch asset data
    const asset = await getAsset(assetId);
    const proof = await getAssetProof(assetId);

    // accounts
    const leafDelegate = asset.ownership.delegate ? new PublicKey(asset.ownership.delegate) : new PublicKey(asset.ownership.owner);
    const proofPathAsAccounts = mapProof(proof);

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