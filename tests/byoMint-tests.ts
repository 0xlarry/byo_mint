// import {Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
// import * as anchor from "@coral-xyz/anchor";
// import base58 from 'bs58';
// import { 
//   findFaucetPda, findMetadataMapPda, 
//   executeTx,
//   addNewTree, createFaucetIx, createMetadataMapIx, deleteMetadataMapIx, updateFaucetIx, mintCnftIx, payoutIx
// } from "./byoMint-client";
// import { ByoMint } from "../target/types/byo_mint";

// describe.only("ByoMint", () => {
//   const rpc = String(process.env.RPC);
//   const connection = new Connection(rpc, "confirmed")
//   const decodedSecretKey = base58.decode(String(process.env.TEST_KEY));
//   const kp = Keypair.fromSecretKey(decodedSecretKey);
//   const decodedSecretKey2 = base58.decode(String(process.env.TEST_KEY1));
//   const kp2 = Keypair.fromSecretKey(decodedSecretKey2);
//   const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(kp), {commitment: 'confirmed', skipPreflight: true}); 
//   anchor.setProvider(provider);
//   const program = anchor.workspace.ByoMint as anchor.Program<ByoMint>;

//   before("airdrop",async () => {
//     console.log("-- kp1 address:", kp.publicKey.toString());
//     console.log("-- kp1 balance: ", await connection.getBalance(kp.publicKey) / LAMPORTS_PER_SOL);
//     console.log("-- kp2 address:", kp2.publicKey.toString());
//     console.log("-- kp2 balance: ", await connection.getBalance(kp2.publicKey) / LAMPORTS_PER_SOL);
//   });

//   const symbol = "BYOG";
//   const metadataMapPda = findMetadataMapPda(program, kp.publicKey, symbol);
//   const faucetPda = findFaucetPda(program, kp.publicKey, metadataMapPda);
//   // console.log({
//   //   mdMapPda: metadataMapPda.toString(),
//   //   faucetPda: faucetPda.toString()
//   // })
//   describe("init faucet", () => {
//     it("should create faucet", async () => {
//       const mdIx = await createMetadataMapIx(
//         program, 
//         kp.publicKey, 
//         500, 
//         [1, 6, 5, 8, 5, 7, 6, 6], 
//         'https://shdw-drive.genesysgo.net/Gz1cS4VEX9v7ep5cnPnqaByMGzF4LudSMCPBr1qS6RFx',
//         symbol
//       );
//       const {ix, collectionMint} = await createFaucetIx(program, kp.publicKey, metadataMapPda, "BYOGnomes", symbol, "https://shdw-drive.genesysgo.net/Gz1cS4VEX9v7ep5cnPnqaByMGzF4LudSMCPBr1qS6RFx/collection.json", LAMPORTS_PER_SOL * 0.01);
//       await executeTx(kp, [mdIx, ix], collectionMint, null, true);
//     });

//     it("should add tree to faucet", async () => {
//       const {ix, emptyMerkleTree, allocTreeIx} = await addNewTree(program, kp.publicKey, faucetPda);
//       await executeTx(kp, [allocTreeIx, ix], emptyMerkleTree);
//     });
//   });
  
//   it.only("should increase supply cap", async () => {
//     const ix = await updateFaucetIx(program, kp.publicKey, faucetPda, null, (LAMPORTS_PER_SOL * 0.01 - LAMPORTS_PER_SOL * 0.00095));
//     await executeTx(kp, [ix], null, false, true);
//   })

//   it("should mint cNFT", async () => {
//     const ix = await mintCnftIx(program, kp2.publicKey, faucetPda, [1, 0, 0, 3, 1, 5, 3, 4, 0, 0]);
//     await executeTx(kp2, [ix], null, false, true);
//   });

//   it("should withdraw fees from faucet", async () => {
//     const ix = await payoutIx(program, kp.publicKey, faucetPda);
//     await executeTx(kp, [ix], null, false, true);
//   });

//   after("show state", async () => {
//     // console.log('-- Metadata Map: ', (await program.account.metadataMap.fetch(metadataMapPda)));
//     // console.log('-- Faucet: ', (await program.account.faucet.fetch(faucetPda)));
//   });
// });

