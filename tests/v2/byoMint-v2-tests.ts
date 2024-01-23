import {Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { ByoMint } from "../../target/types/byo_mint";
import { PublicKey } from "@metaplex-foundation/js";
import { executeTx, addNewTreeV2, createFaucetV2Ix, createLayerMapIx, createOpenMapIx, createSupplyMapIx, findFaucetV2Pda, findLayerMapPda, findOpenMapPda, findSupplyMapPda, loadCliWallet, mintLayerMapIx, mintOpenMapIx, mintSupplyMapIx } from "./byomint-v2-client";

describe.only("ByoMint", () => {
    // SET UP
    const rpc = String(process.env.RPC);
    const connection = new Connection(rpc, "confirmed")
    const kp = loadCliWallet('./test_wallets/test1.json');
    const kp2 = loadCliWallet('./test_wallets/test2.json');
    const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(kp), {commitment: 'confirmed', skipPreflight: true}); 
    anchor.setProvider(provider);
    const program = anchor.workspace.ByoMint as anchor.Program<ByoMint>;

    // TEST DATA
    const symbol = "TEST";
    const sellerFeeBasisPoints = 500;
    const creators = [
        {address: kp.publicKey, share: 50}, 
        {address: new PublicKey('AtNAehxE3MoBKcq7QWjPgP7E3AZoB7SNshSbgeVBXAMy'), share: 50}
    ];
    const creatorsMinter = [
        {address: kp.publicKey, share: 50}, 
        {address: PublicKey.default, share: 50}
    ];
    const uriPrefix = 'http://uri.prefix';
    const items = [
        {name: 'one', jsonUri: 'http://json_uri/one', amount: new anchor.BN(10)},
        {name: 'two', jsonUri: 'http://json_uri/two', amount: new anchor.BN(10)},
        {name: 'three', jsonUri: 'http://json_uri/three', amount: new anchor.BN(10)},
    ];
    const splMint = new PublicKey('8DVvzwVsbetpfgHTw1EWXeJHxQcsZscpheEYnc9DK1FM');

    before("airdrop",async () => {
        console.log("-- kp1 address:", kp.publicKey.toString());
        console.log("-- kp1 balance: ", await connection.getBalance(kp.publicKey) / LAMPORTS_PER_SOL);
        console.log("-- kp2 address:", kp2.publicKey.toString());
        console.log("-- kp2 balance: ", await connection.getBalance(kp2.publicKey) / LAMPORTS_PER_SOL);
    });

    const layerPda = findLayerMapPda(program, kp.publicKey, symbol);
    const supplyPda = findSupplyMapPda(program, kp.publicKey, symbol);
    const openPda = findOpenMapPda(program, kp.publicKey, symbol);
    const layerFaucetPda = findFaucetV2Pda(program, kp.publicKey, new PublicKey('HuojCWSjQQ4Ee266nBNMeZsHzuYWVcYC2yvmxm9mcnNx'));
    const supplyFaucetPda = findFaucetV2Pda(program, kp.publicKey, new PublicKey('3yC7fgQMBAu4n2HJAbhbfHko4HpMwLuP3HxVaBAUTtoG'));
    const openFaucetPda = findFaucetV2Pda(program, kp.publicKey, new PublicKey('B216BM3CztpuHgqe6i1pSCH9MjToHVbRorHfFsZEtWYF'));
    const allFaucetPda = findFaucetV2Pda(program, kp.publicKey, new PublicKey('DhKXtH8YP9UpS3dNZBwxyMTucDij42VbMicvkQVethkn'));
    describe("create metadata maps", () => {
        it("should create layer map", async () => {
            const ix = await createLayerMapIx(
                program,
                kp.publicKey,
                sellerFeeBasisPoints, 
                uriPrefix,
                symbol,
                creators,
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
            );
            await executeTx(kp, [ix], null, null, true);
        });
        it("should create supply map", async () => { // NOT WORKING TODO
            const ix = await createSupplyMapIx(
                program,
                kp.publicKey,
                sellerFeeBasisPoints, 
                symbol,
                creators,
                items
            );
            await executeTx(kp, [ix], null, null, true);
        });
        it("should create open map", async () => {
            const ix = await createOpenMapIx(
                program,
                kp.publicKey,
                sellerFeeBasisPoints, 
                symbol,
                creatorsMinter
            );
            await executeTx(kp, [ix], null, null, true);
        });
    });
    
    describe("create faucet", async () => {
        it("should create faucet with layer map", async () => {
            const {ix, collectionMint} = await createFaucetV2Ix(
                program, 
                kp.publicKey,
                layerPda,
                null,
                null, 
                "Layer Collection",
                "LAY",
                "http://layerMap.com",
                100,
                LAMPORTS_PER_SOL * 0.1,
                null
            );
            await executeTx(kp, [ix], collectionMint, null, true);
            const {ix: treeIx, emptyMerkleTree, allocTreeIx} = await addNewTreeV2(
                program,
                kp.publicKey, 
                layerFaucetPda
            );
            await executeTx(kp, [allocTreeIx, treeIx], emptyMerkleTree, null, true);
        });

        it("should create faucet with supply map", async () => {
            const {ix, collectionMint} = await createFaucetV2Ix(
                program, 
                kp.publicKey,
                null,
                supplyPda,
                null, 
                "Supply Collection",
                "SUP",
                "http://supplyMap.com",
                100,
                LAMPORTS_PER_SOL * 0.1,
                null
            );
            await executeTx(kp, [ix], collectionMint, null, true);
            const {ix: treeIx, emptyMerkleTree, allocTreeIx} = await addNewTreeV2(
                program,
                kp.publicKey, 
                supplyFaucetPda
            );
            await executeTx(kp, [allocTreeIx, treeIx], emptyMerkleTree, null, true);
        });

        it("should create faucet with open map", async () => {
            const {ix, collectionMint} = await createFaucetV2Ix(
                program, 
                kp.publicKey,
                null,
                null,
                openPda, 
                "Open Collection",
                "OPEN",
                "http://openMap.com",
                100,
                LAMPORTS_PER_SOL * 0.1,
                null
            );
            await executeTx(kp, [ix], collectionMint, null, true);
            const {ix: treeIx, emptyMerkleTree, allocTreeIx} = await addNewTreeV2(
                program,
                kp.publicKey, 
                openFaucetPda
            );
            await executeTx(kp, [allocTreeIx, treeIx], emptyMerkleTree, null, true);
        });

        it("should create faucet with ALL maps AND SPL mint", async () => {
            const {ix, collectionMint} = await createFaucetV2Ix(
                program, 
                kp.publicKey,
                layerPda,
                supplyPda,
                openPda, 
                "All Collection",
                "ALL",
                "http://allMaps.com",
                100,
                1,
                splMint
            );
            await executeTx(kp, [ix], collectionMint, null, true);
            const {ix: treeIx, emptyMerkleTree, allocTreeIx} = await addNewTreeV2(
                program,
                kp.publicKey, 
                allFaucetPda
            );
            await executeTx(kp, [allocTreeIx, treeIx], emptyMerkleTree, null, true);
        });
    });

    describe("mint cNFTs", async () => {
        it.only("should mint from layer map", async () => { // STOPPED HERE, "MISSING ACCOUNT"
            const ix = await mintLayerMapIx(
                program,
                kp2.publicKey,
                layerFaucetPda,
                [0,0,0,0,0,0,0,0,0,0],
                null
            );
            await executeTx(kp2, [ix], null, null, true);
        });

        it("should mint from supply map", async () => {
            const ix = await mintSupplyMapIx(
                program,
                kp2.publicKey,
                supplyFaucetPda
            );
            await executeTx(kp2, [ix], null, null, true);
        });

        it("should mint from open map", async () => {
            const ix = await mintOpenMapIx(
                program,
                kp2.publicKey,
                openFaucetPda,
                'name',
                'http://jsonUri.com'
            );
            await executeTx(kp2, [ix], null, null, true);
        });

        it("should mint from all map", async () => {
            let ix = await mintLayerMapIx(
                program,
                kp2.publicKey,
                allFaucetPda,
                [1,0,0,0,0,0,0,0,0,0],
                null
            );
            await executeTx(kp2, [ix], null, null, true);
            ix = await mintSupplyMapIx(
                program,
                kp2.publicKey,
                allFaucetPda
            );
            await executeTx(kp2, [ix], null, null, true);
            ix = await mintOpenMapIx(
                program,
                kp2.publicKey,
                allFaucetPda,
                'name',
                'http://jsonUri.com'
            );
            await executeTx(kp, [ix], null, null, true);
        });
    });

    describe.skip("background", async () => {
        it("should update background color", async () => {

        });

        it("should close background account", async () => {

        });
    });

    after("show state", async () => {
        // console.log('-- Metadata Map: ', (await program.account.metadataMap.fetch(metadataMapPda)));
        // console.log('-- Faucet: ', (await program.account.faucet.fetch(faucetPda)));
        // console.log('-- BGs: ', (await program.account.background.fetch(findBackgroundPda(program, new PublicKey('CibvEBCs9vYPcUraGz4D2kzmnMpwhf36HMHDFkGtkiqa')))));
    });
});