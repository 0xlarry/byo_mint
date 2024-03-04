use mpl_bubblegum::types::Creator;

use crate::*;

#[account]
pub struct MetadataMapV2 { 
    pub authority: Pubkey,
    pub layers: [u8; 10], // array of max 10 layers, u8 -> layer variant, 0 == no trait
    pub seller_fee_basis_points: u16,
    pub symbol: String,
    pub uri_prefix: String,
    pub creators: Vec<ByoCreator>,
}

impl MetadataMapV2 {
    pub const LEN: usize = 8 
        + 32
        + 10    // max of 10 traits
        + 2
        + 4     // symbol
        + 150  // uri max 150 chars
        + 5 * ByoCreator::LEN;       // 5 creators
        
    pub fn new(authority: Pubkey, layers: [u8; 10], sfbp: u16, symbol: String, uri_prefix: String, creators: Vec<ByoCreator>) -> Result<MetadataMapV2> {
        // input validation
        require!(layers.len() <= 10, ByomError::TooManyLayers);
        require!(sfbp <= 10000, ByomError::InvalidRoyalty);
        require!(symbol.len() <= 4, ByomError::InvalidSymbol);
        require!(uri_prefix.len() <= 150, ByomError::InvalidUri);
        ByoCreator::verify_creator_shares(&creators.clone())?;

        Ok(MetadataMapV2 {
            authority,
            layers,
            seller_fee_basis_points: sfbp, 
            symbol,
            uri_prefix,
            creators
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

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ByoCreator {
    pub address: Pubkey,
    pub share: u8             
}
impl ByoCreator {
    pub const LEN: usize = 8 
        + 32
        + 1;
    pub fn verify_creator_shares(creators: &Vec<ByoCreator>) -> Result<()> {
        require!(creators.len() <= 5, ByomError::InvalidRoyalty);
        let total_shares: u16 = creators.iter().map(|x| x.share as u16).sum();
        require!(total_shares == 100, ByomError::InvalidRoyalty);
        Ok(())
    }
    pub fn build_creators(creators: Vec<ByoCreator>, minter_pubkey: Pubkey) -> Vec<Creator> {
        let mut ret_creators: Vec<Creator> = Vec::new();
        for c in creators.iter() {
            let crtr;
            if c.address == Pubkey::default() { // GRANT MINTER royalties
                crtr = minter_pubkey;
            } else {
                crtr = c.address
            }
            ret_creators.push(Creator {
                address: crtr,
                verified: false,
                share: c.share
            });
        }
        return ret_creators;
    }
}