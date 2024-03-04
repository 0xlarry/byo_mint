// import {Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
// import * as anchor from "@coral-xyz/anchor";
// import base58 from 'bs58';
// import { 
//   findMetadataMapPda, 
//   executeTx,
//   createMetadataMapIx,updateFaucetWlIx, payoutWlIx, creatWlFaucetIx, mintWlIx, addNewTreeWl, findFaucetWlPda
// } from "./byoMint-client";
// import { ByoMint } from "../target/types/byo_mint";
// import { PublicKey } from "@metaplex-foundation/js";

// describe.skip("wl Mint", () => {
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

//   const symbol = "SENT";
//   const metadataMapPda = findMetadataMapPda(program, kp.publicKey, symbol);
//   const faucetPda = findFaucetWlPda(program, kp.publicKey, metadataMapPda);
//   console.log({
//     mdMapPda: metadataMapPda.toString(),
//     faucetPda: faucetPda.toString()
//   })
//   describe("init faucet", () => {
//     it("should create faucet", async () => {
//       const mdIx = await createMetadataMapIx(
//         program, 
//         kp.publicKey, 
//         500, 
//         [10, 11, 15, 7, 27, 6], 
//         'https://shdw-drive.genesysgo.net/6vXdefvho6eDhJo7Z6PkN73DiZBvUH4nM7Bpt88pKviq',
//         symbol
//       );
//       const {ix, collectionMint} = await creatWlFaucetIx(
//         program, 
//         kp.publicKey, 
//         metadataMapPda, 
//         "Saga Sentients", 
//         symbol, 
//         "https://shdw-drive.genesysgo.net/6vXdefvho6eDhJo7Z6PkN73DiZBvUH4nM7Bpt88pKviq/collection.json", 
//         (LAMPORTS_PER_SOL * 0.05001 - LAMPORTS_PER_SOL * 0.00095 * 2),
//         new PublicKey('46pcSL5gmjBrPqGKFaLbbCmR6iVuLJbnQy13hAe7s6CC')
//       );
//       await executeTx(kp, [mdIx, ix], collectionMint, null, true);
//     });

//     it("should add tree to faucet", async () => {
//       const {ix, emptyMerkleTree, allocTreeIx} = await addNewTreeWl(program, kp.publicKey, faucetPda);
//       await executeTx(kp, [allocTreeIx, ix], emptyMerkleTree, false, true);
//     });
//   });
  
//   it("should increase supply cap and/or update price", async () => {
//     const ix = await updateFaucetWlIx(program, kp.publicKey, faucetPda, null, (LAMPORTS_PER_SOL * 0.05001 - LAMPORTS_PER_SOL * 0.00095 * 2));
//     await executeTx(kp, [ix], null, false, true);
//   })

//   it("should mint cNFT", async () => {
//     const ix = await mintWlIx(program, kp2.publicKey, faucetPda, new PublicKey("wl_token_goes_here"), [3, 0, 3, 0, 1, 0, 0, 0, 0, 0], "#a2ff1a");
//     await executeTx(kp2, [ix], null, false, true);
//   });

//   it("should withdraw fees from faucet", async () => {
//     const ix = await payoutWlIx(program, kp.publicKey, faucetPda);
//     await executeTx(kp, [ix], null, false, true);
//   });

//   after("show state", async () => {
//     // console.log('-- Metadata Map: ', (await program.account.metadataMap.fetch(metadataMapPda)));
//     // console.log('-- Faucet: ', (await program.account.faucetWl.fetch(faucetPda)));
//   });
// });

