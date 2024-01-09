use crate::*;

#[account]
pub struct SupplyMap { 
    pub authority: Pubkey,
    pub variants: Vec<Variant>,         // max of 20
    pub seller_fee_basis_points: u16,
    pub name: String,
    pub uri_prefix: String
}

impl SupplyMap {
    pub const LEN: usize = 8 
        + 32
        + 4 + (20 * Variant::LEN)
        + 2
        + 25
        + 150;  // uri max 150 chars
        
    pub fn new(authority: Pubkey, variants: Vec<Variant>, sfbp: u16, uri_prefix: String, name: String) -> Result<SupplyMap> {
        // input validation
        require!(sfbp <= 10000, ByomError::InvalidRoyalty);
        require!(uri_prefix.len() <= 150, ByomError::InvalidUri);
        require!(variants.len() <= 20, ByomError::TooManyVariants);
        SupplyMap::verify_variants(variants.clone())?;

        Ok(SupplyMap {
            authority,
            variants,
            seller_fee_basis_points: sfbp, 
            uri_prefix,
            name
        })
    }

    pub fn verify_variants(variants: Vec<Variant>) -> Result<()> {
        for variant in variants.iter() {
            require!(
                variant.name.len() <= 32 && variant.symbol.len() <= 4 && variant.amount > 0
            , ByomError::InvalidVariant);
        }
        Ok(())
    }

    pub fn select_variant(&mut self, clock: &Sysvar<Clock>) -> Result<Variant> {
        if self.variants.is_empty() {
            return Err(ByomError::NothingToMint.into());
        }
    
        let total_amount: u64 = self.variants.iter().map(|v| v.amount as u64).sum();
        let rng = get_random_index(clock, 0, total_amount - 1);
    
        let mut accumulated_amount: u64 = 0;
        for v in self.variants.iter() {
            accumulated_amount += v.amount as u64;
            if rng <= accumulated_amount {
                return Ok(v.clone());
            }
        }
        return Err(ByomError::NothingToMint.into())
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Variant {
    name: String,   // max 32
    symbol: String, // max 4
    amount: u16
}
impl Variant {
    pub const LEN: usize = 8 
        + 32 // 32 char name
        + 4 // 4 char symbol
        + 2;

    pub fn new(name: String, symbol: String, amount: u16) -> Variant {
        Variant { name, symbol, amount }
    }
}