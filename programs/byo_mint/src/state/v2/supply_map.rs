use crate::*;

#[account]
pub struct SupplyMap { 
    pub authority: Pubkey,
    pub seller_fee_basis_points: u16,
    pub symbol: String, // max 4
    pub uri_prefix: String, // max 100
    pub creators: Vec<(Pubkey, u8)>,
    pub items: Vec<Item>,
}


impl SupplyMap {
    pub const LEN: usize = 8 
        + 32
        + 2
        + 4
        + 100
        + 4 + 25 * Item::LEN; // max of 25 items
    
    pub fn new(authority: Pubkey, sfbp: u16, symbol: String, uri_prefix: String, creators: &Vec<(Pubkey, u8)>, items: Vec<Item>) -> Result<SupplyMap> {
        // input validation
        require!(items.len() <= 25, ByomError::TooManyItems);
        require!(sfbp <= 10000, ByomError::InvalidRoyalty);
        require!(symbol.len() <= 4, ByomError::InvalidSymbol);
        require!(uri_prefix.len() <= 150, ByomError::InvalidUri);

        verify_creator_shares(creators)?;

        Ok(SupplyMap {
            authority,
            seller_fee_basis_points: sfbp, 
            symbol,
            uri_prefix, 
            creators: creators.clone(),
            items
        })
    }

    pub fn select_item(&mut self, clock: &Sysvar<Clock>) -> Result<Item> {
        if self.items.is_empty() {
            return Err(ByomError::NothingToMint.into());
        }
    
        let total_amount: u64 = self.items.iter().map(|item| item.amount).sum();
        let rng = get_random_index(clock, 0, total_amount - 1, 13); // need arbitrary seed as there is only 1 randomization in this ix
    
        let mut accumulated_amount = 0;
        for item in self.items.iter_mut() {
            accumulated_amount += item.amount;
            if rng <= accumulated_amount {
                item.amount -= 1;
                return Ok(item.clone());
            }
        }
        return Err(ByomError::NothingToMint.into())
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Item {
    pub name: String,               // 15
    pub json_uri_suffix: String,    // 25
    pub amount: u64,                
}

impl Item {
    pub const LEN: usize = 8 
        + 15 // 15 char name
        + 25 // json uri
        + 8;

    pub fn new(name: String, json_uri_suffix: String, amount: u64) -> Item {
        Item { name, json_uri_suffix, amount }
    }
}