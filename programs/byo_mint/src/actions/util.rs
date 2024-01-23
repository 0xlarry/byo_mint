use crate::*;
use anchor_lang::solana_program::{
    clock::Clock,
    hash::Hasher
};
use mpl_bubblegum::types::Creator;

pub fn is_valid_hex_color(input: &str) -> bool {
    // Check if the input starts with '#' and has a length of 7
    if input.len() == 7 && input.starts_with('#') {
        // Check if all characters following '#' are hexadecimal
        input[1..].chars().all(|c| c.is_digit(16))
    } else {
        false
    }
}

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