use crate::*;

#[account]
pub struct AdditionalAssets {
    pub background: Pubkey,
    pub trait_replacements: [Pubkey; 10]
}

impl AdditionalAssets {
    pub const LEN: usize = 8
        + 32
        + 10*32 + 1;
    
    pub fn new() -> AdditionalAssets {
        AdditionalAssets {
            background: Pubkey::default(),
            trait_replacements: [Pubkey::default(); 10]
        }
    }
}