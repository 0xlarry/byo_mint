use crate::*;
use anchor_spl::token::TokenAccount;
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};

#[account]
pub struct FaucetWl {
    pub authority: Pubkey, 
    pub collection_mint: Pubkey,
    pub merkle_tree: Pubkey,
    pub metadata_map: Pubkey,
    pub current_supply: u64,
    pub supply_cap: u64,
    pub mint_price: u64,
    pub wl_collection: Pubkey,
    pub bump: u8,
}


impl FaucetWl {
    pub const LEN: usize = 8 
        + 32 
        + 32 
        + 32
        + 32 
        + 8
        + 8
        + 8
        + 32
        + 1;

    pub fn assert_wl(&mut self, signer: Pubkey, token_account: Account<TokenAccount>, metadata: AccountInfo) -> Result<()> {
        let metadata: Metadata = Metadata::from_account_info(&metadata)?;
        // assert ownership
        require!(token_account.owner == signer && token_account.amount == 1 && token_account.mint == metadata.mint, ByomError::InvalidOwner);
        // assert collection
        match &metadata.collection {
            None => return Err(ByomError::InvalidCollection.into()),
            Some (md_collection) => {
                msg!(&md_collection.key.to_string());
                require!(md_collection.verified, ByomError::InvalidCollection); // collection must be verified
                require!(md_collection.key == self.wl_collection, ByomError::InvalidCollection); // must match the 
                msg!("-- WL VALID!");
            }
        }
        Ok(())
    }
}