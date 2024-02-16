use crate::*;

#[account]
pub struct Faucet {
    pub authority: Pubkey, 
    pub collection_mint: Pubkey,
    pub merkle_tree: Pubkey,
    pub metadata_map: Pubkey,
    pub current_supply: u64,
    pub supply_cap: u64,
    pub mint_price: u64,
    pub bump: u8,
}


impl Faucet {
    pub const LEN: usize = 8 
        + 32 
        + 32 
        + 32
        + 32 
        + 8
        + 8
        + 8
        + 1;
}