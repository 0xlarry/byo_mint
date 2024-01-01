use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    clock::Clock,
    hash::Hasher,
};

pub fn generate_random_int(clock: &Sysvar<Clock>, seed: String) -> u8 {
    let seed1 = "YouWillNevaGetDis";
    let seed2 = "KingOfDaCastle";
    let timestamp = clock.unix_timestamp;
    
    let mut hasher_state = Hasher::default();
    hasher_state.hashv(&[
        &timestamp.to_le_bytes(),
        seed.as_bytes(),
        seed1.as_bytes(),
        seed2.as_bytes()
    ]);
    let hash_result = hasher_state.result();

    let hash_slice = &hash_result.as_ref()[0..8];
    let random_u64 = u64::from_le_bytes(hash_slice.try_into().expect("Slice length mismatch"));
    (random_u64 / u64::MAX) as u8
}