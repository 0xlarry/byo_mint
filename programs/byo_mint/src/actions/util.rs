use crate::*;
use anchor_lang::solana_program::{
    clock::Clock,
    hash::Hasher,
    account_info::AccountInfo
};
use mpl_bubblegum::{
    instructions::VerifyLeafCpiBuilder, 
    utils::get_asset_id, 
    types::{
      LeafSchema,
      Creator, 
    }
};

pub fn is_valid_hex_color(input: &str) -> bool {
    // Check if the input starts with '#' and has a length of 7
    if input.len() == 7 && input.starts_with('#') {
        // Check if all characters following '#' are hexadecimal
        input[1..].chars().all(|c| c.is_digit(16))
    } else {
        false
    }
}

pub fn check_cnft_owner<'a>(
    leaf_owner: &AccountInfo<'a>,
    merkle_tree: &AccountInfo<'a>,
    compression_program: &AccountInfo<'a>,
    remaining_proof: &[AccountInfo<'a>],
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32) -> Result<()> {
    let asset_id = get_asset_id(&merkle_tree.key(), nonce);

    let leaf = LeafSchema::V1 {
        id: asset_id,
        owner: leaf_owner.key(),
        delegate: leaf_owner.key(), // leaf delegate
        nonce,
        data_hash,
        creator_hash,
    };

    let remaining_accounts: Vec<(&AccountInfo, bool, bool)> = remaining_proof
        .iter()
        .map(|account| (account, account.is_signer,account.is_writable))
        .collect();

    // this panics in case the verification fails - and we just let it fall through
    VerifyLeafCpiBuilder::new(compression_program)
        .merkle_tree(merkle_tree)
        .add_remaining_accounts(&remaining_accounts)
        .root(root)
        .leaf(leaf.hash())
        .index(index)
        .invoke()?;

    Ok(())
}

// fn validate_metadata(data_hash: [u8; 32], name: String, uri: String, symbol: String) -> Result<()> {
//     let md =  MetadataArgs {
//         name: name.clone(),
//         symbol: symbol.clone(),
//         uri: uri.clone(),
//         creators: vec![
//             Creator {address: <Pubkey as std::str::FromStr>::from_str("7NKeWP9wA288NcD5CXF5cZQNCGonwt3yRPPYNp2LtXch").unwrap(), verified: true, share: 100}
//         ],
//         seller_fee_basis_points: 500,
//         primary_sale_happened: false,
//         is_mutable: false,
//         edition_nonce: Some(0),
//         uses: None,
//         collection: Some(Collection {verified: true, key: <Pubkey as std::str::FromStr>::from_str("GNioYiqi1TGWiSLuTHb3Xx1rXZGeco6hC5AU7V4bKApb").unwrap()}),
//         token_program_version: TokenProgramVersion::Original,
//         token_standard: Some(TokenStandard::NonFungible),
//     };
//     let incoming_data_hash = hash_metadata(&md)?;
//     require!(data_hash == incoming_data_hash, ByomError::InvalidCollection);
//     Ok(())
// }
// pub fn hash_metadata(metadata: &MetadataArgs) -> Result<[u8; 32]> {
//     let metadata_args_hash = keccak::hashv(&[&metadata.try_to_vec().unwrap()]);
//     // Calculate new data hash.
//     Ok(keccak::hashv(&[
//         &metadata_args_hash.to_bytes(),
//         &metadata.seller_fee_basis_points.to_le_bytes(),
//     ])
//     .to_bytes())
// }

pub fn generate_random_int(clock: &Sysvar<Clock>, seed: u8) -> u64 {
    let seed1 = "YouWillNevaGetDis";
    let seed2 = "KingOfDaCastle";
    let timestamp = clock.unix_timestamp;
    
    let mut hasher_state = Hasher::default();
    hasher_state.hashv(&[
        &timestamp.to_le_bytes(),
        &[seed],
        seed1.as_bytes(),
        seed2.as_bytes(),
    ]);
    let hash_result = hasher_state.result();

    let hash_slice = &hash_result.as_ref()[0..8];
    let random_u64 = u64::from_le_bytes(hash_slice.try_into().expect("Slice length mismatch"));
    random_u64
}

pub fn get_random_index(clock: &Sysvar<Clock>, min: u64, max: u64, seed: u8) -> u64 {
    let random_u8 = generate_random_int(clock, seed);
    random_u8 % (max - min + 1)
}

pub fn verify_creator_shares(creators: &Vec<(Pubkey, u8)>) -> Result<()> {
    require!(creators.len() <= 5, ByomError::InvalidRoyalty);
    let total_shares: u16 = creators.iter().map(|&(_, x)| x as u16).sum();
    require!(total_shares == 100, ByomError::InvalidRoyalty);
    Ok(())
}

pub fn build_creators(creators: Vec<(Pubkey, u8)>, minter_pubkey: Pubkey) -> Vec<Creator> {
    let mut ret_creators: Vec<Creator> = Vec::new();
    for c in creators.iter() {
        let crtr;
        if c.0 == Pubkey::default() { // GRANT MINTER royalties
            crtr = minter_pubkey;
        } else {
            crtr = c.0
        }
        ret_creators.push(Creator {
            address: crtr,
            verified: false,
            share: c.1
        });
    }
    return ret_creators;
}
