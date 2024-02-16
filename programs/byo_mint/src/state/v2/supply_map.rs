use crate::*;

#[account]
pub struct SupplyMap { 
    pub authority: Pubkey,
    pub seller_fee_basis_points: u16,
    pub symbol: String, // max 4
    pub creators: Vec<ByoCreator>,
    pub items: Vec<Item>,
}


impl SupplyMap {
    pub const LEN: usize = 8 
        + 32
        + 2
        + 4
        + 5 * ByoCreator::LEN
        + 4 + 10 * Item::LEN; // max of 10 items
    
    pub fn new(authority: Pubkey, sfbp: u16, symbol: String, creators: &Vec<ByoCreator>, items: Vec<Item>) -> Result<SupplyMap> {
        // input validation
        require!(sfbp <= 10000, ByomError::InvalidRoyalty);
        require!(symbol.len() <= 4, ByomError::InvalidSymbol);
        Item::verify_items(items.clone())?;
        verify_creator_shares(creators)?;

        Ok(SupplyMap {
            authority,
            seller_fee_basis_points: sfbp, 
            symbol,
            creators: creators.clone(),
            items
        })
    }

    pub fn select_item(&mut self, clock: &Sysvar<Clock>) -> Result<Item> {
        let total_amount: u64 = self.items.iter().map(|item| item.amount).sum();
        if self.items.is_empty() || total_amount == 0 {
            return Err(ByomError::NothingToMint.into());
        }
        let rng = get_random_index(clock, 0, total_amount - 1, 13); // need arbitrary seed as there is only 1 randomization in this ix
    
        let mut accumulated_amount = 0;
        for item in self.items.iter_mut() {
            accumulated_amount += item.amount;
            if rng <= accumulated_amount {
                if item.amount == 0 {
                    continue;
                }
                msg!("ITEM: {:?}", item);
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
    pub json_uri: String,           // 150
    pub amount: u64,                
}

impl Item {
    pub const LEN: usize = 8 
        + 15 // 15 char name
        + 150 // json uri
        + 8;

    pub fn new(name: String, json_uri: String, amount: u64) -> Item {
        Item { name, json_uri, amount }
    }

    pub fn verify_items(items: Vec<Item>) -> Result<()> {
        require!(items.len() <= 10, ByomError::TooManyItems);
        for i in items.iter() {
            require!(i.name.len() <= 15, ByomError::InvalidName);
            require!(i.json_uri.len() <= 150, ByomError::InvalidUri);
        }
        Ok(())
    }
}