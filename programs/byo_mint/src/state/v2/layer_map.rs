use crate::*;

#[account]
pub struct LayerMap { 
    pub authority: Pubkey,
    pub seller_fee_basis_points: u16,
    pub symbol: String,
    pub uri_prefix: String,
    pub creators: Vec<ByoCreator>,
    pub layers: [u8; 10], // array of max 10 layers, u8 -> layer variant, 0 == no trait
}

impl LayerMap {
    pub const LEN: usize = 8 
        + 32
        + 2
        + 4                     // symbol
        + 150                   // uri max 150 chars
        + 5 * ByoCreator::LEN       // 5 creators
        + 10                    // 10 traits
        + 1 + 32
        + 10;
        
    pub fn new(authority: Pubkey, sfbp: u16, symbol: String, uri_prefix: String, creators: &Vec<ByoCreator>, layers: [u8; 10]) -> Result<LayerMap> {
        // input validation
        require!(layers.len() <= 10, ByomError::TooManyLayers);
        require!(sfbp <= 10000, ByomError::InvalidRoyalty);
        require!(symbol.len() <= 4, ByomError::InvalidSymbol);
        require!(uri_prefix.len() <= 150, ByomError::InvalidUri);

        verify_creator_shares(creators)?;

        Ok(LayerMap {
            authority,
            seller_fee_basis_points: sfbp, 
            symbol,
            uri_prefix, 
            creators: creators.clone(),
            layers,
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