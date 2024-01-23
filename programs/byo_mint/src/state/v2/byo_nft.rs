use crate::*;

#[account]
pub struct ByoNft {
    pub trait_combo: Pubkey,
    pub bg_color: Option<String>,
    pub bg_asset: Option<Pubkey>,
    pub trait_replacements: [Pubkey; 10]
}

impl ByoNft {
    pub const LEN: usize = 8
        + 32
        + 6 + 1
        + 32 + 1
        + 32 * 10;

    pub fn new(trait_combo: Pubkey, bg_color: Option<String>) -> ByoNft {
        ByoNft {
            trait_combo,
            bg_color, 
            bg_asset: None, 
            trait_replacements: [Pubkey::default(); 10]
        }
    }
}