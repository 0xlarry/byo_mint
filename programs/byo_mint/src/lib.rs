use std::str::FromStr;

use anchor_lang::prelude::*;
pub mod byom_error;
pub use byom_error::*;
pub mod state;
pub use state::*;
pub mod actions;
pub use actions::*;
declare_id!("GnukyHhbXhqeswaZbzDiTLZeMSHMuWNTNxw4buhS6zpg");

#[program]
pub mod byo_mint {
    use super::*;

    // *************************************************************
    // METADATA MAP
    // *************************************************************
    pub fn create_metadata_map<'info>( // depricated v0
        ctx: Context<'_, '_, '_, 'info, CreateMetadataMap<'info>>,
        params: CreateMetadataMapParams
    ) -> Result<()> {
        actions::create_metadata_map(ctx, params)
    }

    pub fn delete_metadata_map<'info>( // depricated v0
        ctx: Context<'_, '_, '_, 'info, DeleteMetadataMap<'info>>
    ) -> Result<()> {
        actions::delete_metadata_map(ctx)
    }

    // byo map
    pub fn create_byo_map<'info>( // depricated v0
        ctx: Context<'_, '_, '_, 'info, CreateByoMap<'info>>,
        params: CreateByoMapParams
    ) -> Result<()> {
        actions::create_byo_map(ctx, params)
    }

    pub fn delete_byo_map<'info>( // depricated v0
        ctx: Context<'_, '_, '_, 'info, DeleteByoMap<'info>>
    ) -> Result<()> {
        actions::delete_byo_map(ctx)
    }
    // supply map
    pub fn create_supply_map<'info>( // depricated v0
        ctx: Context<'_, '_, '_, 'info, CreateSupplyMap<'info>>,
        params: CreateSupplyMapParams
    ) -> Result<()> {
        actions::create_supply_map(ctx, params)
    }

    pub fn delete_supply_map<'info>( // depricated v0
        ctx: Context<'_, '_, '_, 'info, DeleteSupplyMap<'info>>
    ) -> Result<()> {
        actions::delete_supply_map(ctx)
    }

    // *************************************************************
    // FAUCET
    // *************************************************************
    pub fn create_faucet<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateFaucet<'info>>,
        params: CreateFaucetParams
    ) -> Result<()> {
        actions::create_faucet(ctx, params)
    }

    pub fn add_new_tree<'info>(
        ctx: Context<'_, '_, '_, 'info, AddNewTree<'info>>
    ) -> Result<()> {
        actions::faucet::add_new_tree(ctx)
    }

    pub fn mint<'info>(
        ctx: Context<'_, '_, '_, 'info, MintCnft<'info>>,
        params: MintCnftParams
    ) -> Result<()> {
        actions::byo_cnft(ctx, params)
    }

    pub fn update_faucet<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateFaucet<'info>>,
        params: UpdateFaucetParams
    ) -> Result<()> {
        actions::update_faucet(ctx, params)
    }

    pub fn withdraw_fees<'info>(
        ctx: Context<'_, '_, '_, 'info, WithdrawFees<'info>>
    ) -> Result<()> {
        actions::withdraw_fees(ctx)
    }
}

#[derive(Clone)]
pub struct MplBubblegum;
impl Id for MplBubblegum {
    fn id() -> Pubkey {
        mpl_bubblegum::ID
    }
}

#[derive(Clone)]
pub struct MplTokenMetadata;
impl Id for MplTokenMetadata {
    fn id() -> Pubkey {
        mpl_token_metadata::ID
    }
}

#[derive(Clone)]
pub struct Noop;
impl Id for Noop {
    fn id() -> Pubkey {
        Pubkey::from_str("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV").unwrap()
    }
}

#[derive(Clone)]
pub struct SplAccountCompression;
impl Id for SplAccountCompression {
    fn id() -> Pubkey {
        Pubkey::from_str("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK").unwrap()
    }
}