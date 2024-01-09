use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    clock::Clock,
    hash::Hasher,
};

pub fn generate_random_int(clock: &Sysvar<Clock>) -> u64 {
    let seed1 = "YouWillNevaGetDis";
    let seed2 = "KingOfDaCastle";
    let timestamp = clock.unix_timestamp;
    
    let mut hasher_state = Hasher::default();
    hasher_state.hashv(&[
        &timestamp.to_le_bytes(),
        seed1.as_bytes(),
        seed2.as_bytes()
    ]);
    let hash_result = hasher_state.result();

    let hash_slice = &hash_result.as_ref()[0..8];
    let random_u64 = u64::from_le_bytes(hash_slice.try_into().expect("Slice length mismatch"));
    random_u64 / u64::MAX
}

pub fn get_random_index(clock: &Sysvar<Clock>, min: u64, max: u64) -> u64 {
    let random_u64 = generate_random_int(clock);
    random_u64 % (max - min + 1)
}

pub fn is_valid_hex_color(input: &str) -> bool {
    // Check if the input starts with '#' and has a length of 7
    if input.len() == 7 && input.starts_with('#') {
        // Check if all characters following '#' are hexadecimal
        input[1..].chars().all(|c| c.is_digit(16))
    } else {
        false
    }
}