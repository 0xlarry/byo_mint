import {Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import base58 from 'bs58';

import { ByoMint } from "../../target/types/byo_mint";
import { PublicKey } from "@metaplex-foundation/js";
import { createMetadataMapV2Ix, addNewTreeV2Ix, createFaucetV2Ix, findFaucetV2Pda, findMetadataMapV2Pda, mintV2Ix, updateFaucetV2Ix } from "./byoMint-v2-client";
import { executeTx } from "../byoMint-client";

describe.only("v2 byo mint program", () => {
  const rpc = String(process.env.RPC);
  const connection = new Connection(rpc, "confirmed")
  const decodedSecretKey = base58.decode(String(process.env.ADMIN_KEY));
  const kp = Keypair.fromSecretKey(decodedSecretKey);
  const decodedSecretKey2 = base58.decode(String(process.env.CLIENT_KEY));
  const kp2 = Keypair.fromSecretKey(decodedSecretKey2);
  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(kp), {commitment: 'confirmed', skipPreflight: true}); 
  anchor.setProvider(provider);
  const program = anchor.workspace.ByoMint as anchor.Program<ByoMint>;

  before("airdrop",async () => {
    console.log("-- kp1 address:", kp.publicKey.toString());
    console.log("-- kp1 balance: ", await connection.getBalance(kp.publicKey) / LAMPORTS_PER_SOL);
    console.log("-- kp2 address:", kp2.publicKey.toString());
    console.log("-- kp2 balance: ", await connection.getBalance(kp2.publicKey) / LAMPORTS_PER_SOL);
  });

  const symbol = "EPD"; //"EDAO"
  const name = "EmpireDao Membership";
  const uriPrefix = 'https://shdw-drive.genesysgo.net/61GxJ6svxHqYuMbCt8W2h6RrjyxfZxKdyAHm7qCfK1PD';
  const collectionUri = `${uriPrefix}/collection.json`;
  const initialSupply = 1;
  const metadataMapPda = findMetadataMapV2Pda(program, kp.publicKey, symbol);
  const faucetPda = findFaucetV2Pda(program, kp.publicKey, metadataMapPda);
  const creators = [{address: new PublicKey('13N8iLGpbVRrRBgKQKmDyVFLL2C3m2UGQyhEi43p52Hc'), share: 100}];
  console.table({
    mdMapPda: metadataMapPda.toString(),
    faucetPda: faucetPda.toString()
  })
  describe.skip("init faucet", () => {
    it("should create faucet", async () => {
      const mdIx = await createMetadataMapV2Ix(
        program, 
        kp.publicKey, 
        500, 
        [35, 8, 12, 15, 11, 27, 0, 0, 0, 0], 
        uriPrefix,
        symbol,
        creators
      );
      const {ix, collectionMint} = await createFaucetV2Ix(
        program, 
        kp.publicKey, 
        metadataMapPda, 
        name, 
        symbol, 
        collectionUri, 
        LAMPORTS_PER_SOL * .01 - (0.00094656 * LAMPORTS_PER_SOL),
        initialSupply
      );
      await executeTx(kp, [mdIx, ix], collectionMint, null, true);
    });

    it("should add tree to faucet", async () => {
      const {ix, emptyMerkleTree, allocTreeIx} = await addNewTreeV2Ix(program, kp.publicKey, faucetPda);
      await executeTx(kp, [allocTreeIx, ix], emptyMerkleTree, false, true);
    });
  });
  
  it.only("should increase supply cap and/or update price", async () => {
    const ix = await updateFaucetV2Ix(program, kp.publicKey, faucetPda, 102, LAMPORTS_PER_SOL * .01 - (0.00094656 * LAMPORTS_PER_SOL));
    await executeTx(kp, [ix], null, false, true);
  })

  it.skip("should mint cNFT", async () => {
    const ix = await mintV2Ix(program, kp2.publicKey, faucetPda, [10, 4, 8, 8, 8, 5, 0, 0, 0, 0], "drawn");
    await executeTx(kp2, [ix], null, false, true);
  });

  after("show state", async () => {
    // console.log('-- Metadata Map: ', (await program.account.metadataMapV2.fetch(metadataMapPda)));
    // console.log('-- Faucet: ', (await program.account.faucetV2.fetch(faucetPda)));
  });
});

