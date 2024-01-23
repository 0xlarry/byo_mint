use crate::*;

#[account]
pub struct FaucetV2 {
    pub authority: Pubkey, 
    pub collection_mint: Pubkey,
    pub merkle_tree: Pubkey,
    pub current_supply: u64,
    pub supply_cap: u64,
    pub mint_price: u64,
    pub layer_map: Pubkey,
    pub supply_map: Pubkey,
    pub mint_token: Pubkey,
    pub wl_collection: Pubkey, 
    pub bump: u8,
}


impl FaucetV2 {
    pub const LEN: usize = 8 
        + 32 
        + 32 
        + 32
        + 32 
        + 32 
        + 8
        + 8
        + 8
        + 32
        + 32
        + 1;

    pub fn new(
        authority: Pubkey, 
        collection_mint: Pubkey,
        merkle_tree: Pubkey,
        supply_cap: u64,
        mint_price: u64,
        layer_map: &Option<Account<LayerMap>>,
        supply_map: &Option<Account<SupplyMap>>,
        mint_token: Pubkey,
        wl_collection: Pubkey, 
        bump: u8,
    ) -> Result<FaucetV2> {
        // ensure faucet auth == md map auths
        let lm = match layer_map {
            Some(x) => {
                require!(x.authority == authority, ByomError::InvalidAuthority);
                x.key()
            },
            None => Pubkey::default()
        };
        let sm = match supply_map {
            Some(x) => {
                require!(x.authority == authority, ByomError::InvalidAuthority);
                x.key()
            },
            None => Pubkey::default()
        };

        Ok(FaucetV2 {
            authority,
            collection_mint, 
            merkle_tree, 
            current_supply: 0,
            supply_cap,
            mint_price, 
            layer_map: lm,
            supply_map: sm,
            mint_token,
            wl_collection,
            bump
        })
    }
}