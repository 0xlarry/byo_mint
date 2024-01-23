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
    pub fn create_metadata_map<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateMetadataMap<'info>>,
        params: CreateMetadataMapParams
    ) -> Result<()> {
        actions::create_metadata_map(ctx, params)
    }

    pub fn delete_metadata_map<'info>(
        ctx: Context<'_, '_, '_, 'info, DeleteMetadataMap<'info>>
    ) -> Result<()> {
        actions::delete_metadata_map(ctx)
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

    pub fn mint<'info>(
        ctx: Context<'_, '_, '_, 'info, MintCnft<'info>>,
        params: MintCnftParams
    ) -> Result<()> {
        actions::byo_cnft(ctx, params)
    }

    // *************************************************************
    // Whitelisted collections, 1:1 per nft in collection
    // *************************************************************
    pub fn create_faucet_wl<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateFaucetWl<'info>>,
        params: CreateFaucetWlParams
    ) -> Result<()> {
        actions::create_faucet_wl(ctx, params)
    }

    pub fn mint_wl<'info>(
        ctx: Context<'_, '_, '_, 'info, MintCnftWl<'info>>,
        params: MintCnftWlParams
    ) -> Result<()> {
        actions::mint_cnft_wl(ctx, params)
    }

    pub fn add_new_tree_wl<'info>(
        ctx: Context<'_, '_, '_, 'info, AddNewTreeWl<'info>>
    ) -> Result<()> {
        actions::faucet::add_new_tree_wl(ctx)
    }

    pub fn update_faucet_wl<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateFaucetWl<'info>>,
        params: UpdateFaucetWlParams
    ) -> Result<()> {
        actions::update_faucet_wl(ctx, params)
    }

    pub fn withdraw_fees_wl<'info>(
        ctx: Context<'_, '_, '_, 'info, WithdrawFeesWl<'info>>
    ) -> Result<()> {
        actions::withdraw_fees_wl(ctx)
    }

    // *************************************************************
    // V2
    // *************************************************************
    // metadata maps
    pub fn create_layer_map<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateLayerMap<'info>>,
        params: CreateLayerMapParams
    ) -> Result<()> {
        actions::create_layer_map(ctx, params)
    }

    pub fn create_supply_map<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateSupplyMap<'info>>,
        params: CreateSupplyMapParams
    ) -> Result<()> {
        actions::create_supply_map(ctx, params)
    }
    // facuet
    pub fn create_faucet_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateFaucetV2<'info>>,
        params: CreateFaucetV2Params
    ) -> Result<()> {
        actions::create_faucet_v2(ctx, params)
    }

    pub fn add_new_tree_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, AddNewTreeV2<'info>>
    ) -> Result<()> {
        actions::add_new_tree_v2(ctx)
    }

    pub fn update_faucet_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateFaucetV2<'info>>,
        params: UpdateFaucetV2Params
    ) -> Result<()> {
        actions::update_faucet_v2(ctx, params)
    }
    // mint
    pub fn mint_layer_map<'info>(
        ctx: Context<'_, '_, '_, 'info, MintLayerMap<'info>>,
        params: MintLayerMapParams
    ) -> Result<()> {
        actions::mint_layer_map(ctx, params)
    }

    pub fn mint_supply_map<'info>(
        ctx: Context<'_, '_, '_, 'info, MintSupplyMap<'info>>,
    ) -> Result<()> {
        actions::mint_supply_map(ctx)
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