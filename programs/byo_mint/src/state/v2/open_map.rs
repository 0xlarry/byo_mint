use crate::*;

#[account]
pub struct OpenMap { 
    pub authority: Pubkey,
    pub seller_fee_basis_points: u16,
    pub symbol: String,
    pub creators: Vec<ByoCreator>,
}

impl OpenMap {
    pub const LEN: usize = 8 
        + 32
        + 2
        + 4                     // symbol
        + 5 * ByoCreator::LEN;       // 5 creators
        
    pub fn new(authority: Pubkey, sfbp: u16, symbol: String, creators: &Vec<ByoCreator>) -> Result<OpenMap> {
        // input validation
        require!(sfbp <= 10000, ByomError::InvalidRoyalty);
        require!(symbol.len() <= 4, ByomError::InvalidSymbol);

        verify_creator_shares(creators)?;

        Ok(OpenMap {
            authority,
            seller_fee_basis_points: sfbp, 
            symbol,
            creators: creators.clone()
        })
    }

    pub fn verify_metadata(json_uri: String, name: String) -> Result<()> {
        require!(json_uri.len() <= 150, ByomError::InvalidUri);
        require!(name.len() <= 25, ByomError::InvalidName);
        Ok(())
    }
}