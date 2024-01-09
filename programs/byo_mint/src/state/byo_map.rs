use crate::*;

#[account]
pub struct ByoMap { 
    pub authority: Pubkey,
    pub layers: [u8; 10], // array of max 10 layers, u8 -> layer variant, 0 == no trait
    pub seller_fee_basis_points: u16,
    pub symbol: String,
    pub uri_prefix: String
}

impl ByoMap {
    pub const LEN: usize = 8 
        + 32
        + 10    // max of 10 traits
        + 2
        + 4     // symbol
        + 150;  // uri max 150 chars
        
    pub fn new(authority: Pubkey, layers: [u8; 10], sfbp: u16, symbol: String, uri_prefix: String) -> Result<ByoMap> {
        // input validation
        require!(layers.len() <= 10, ByomError::TooManyLayers);
        require!(sfbp <= 10000, ByomError::InvalidRoyalty);
        require!(symbol.len() <= 4, ByomError::InvalidSymbol);
        require!(uri_prefix.len() <= 150, ByomError::InvalidUri);

        Ok(ByoMap {
            authority,
            layers,
            seller_fee_basis_points: sfbp, 
            symbol,
            uri_prefix
        })
    }

    pub fn validate_input_layers(&mut self, i_layers: [u8; 10]) -> Result<[u8; 10]> {
        for i in 0..i_layers.len() {
            if i_layers[i] > self.layers[i] {
                msg!("{:?}", i_layers);
                msg!("{:?}", self.layers);
                return Err(ByomError::InvalidVariant.into());
            }
        }
        Ok(i_layers)
    }
}