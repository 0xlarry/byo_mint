use std::str::FromStr;
use anchor_lang::prelude::*;
pub mod byom_error;
pub use byom_error::*;
pub mod state;
pub use state::*;
pub mod actions;
pub use actions::*;
declare_id!("CJBmyuBTAHR8J11C4igZ5bK89moCSUbQs8gdLNavo2o9");

#[program]
pub mod byo_mint {
    use super::*;

    // *************************************************************
    // V1
    // *************************************************************
    // METADATA MAP
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
    // FAUCET
    pub fn create_faucet<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateFaucet<'info>>,
        params: CreateFaucetParams
    ) -> Result<()> {
        actions::create_faucet(ctx, params)
    }
    pub fn add_new_tree<'info>(
        ctx: Context<'_, '_, '_, 'info, AddNewTree<'info>>
    ) -> Result<()> {
        actions::add_new_tree(ctx)
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
    // MINT
    pub fn mint<'info>(
        ctx: Context<'_, '_, '_, 'info, MintCnft<'info>>,
        params: MintCnftParams
    ) -> Result<()> {
        actions::byo_cnft(ctx, params)
    }
    // WHITELIST
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
        actions::add_new_tree_wl(ctx)
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
    // METADATA MAPS
    pub fn init_layer_map<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateLayerMap<'info>>,
        params: CreateLayerMapParams
    ) -> Result<()> {
        actions::create_layer_map(ctx, params)
    }
    pub fn init_supply_map<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateSupplyMap<'info>>,
        params: CreateSupplyMapParams
    ) -> Result<()> {
        actions::create_supply_map(ctx, params)
    }
    pub fn edit_supply<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateSupply<'info>>,
        params: UpdateSupplyParams
    ) -> Result<()> {
        actions::update_supply(ctx, params)
    }
    pub fn init_open_map<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateOpenMap<'info>>,
        params: CreateOpenMapParams
    ) -> Result<()> {
        actions::create_open_map(ctx, params)
    }
    // FAUCET
    pub fn init_faucet_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateFaucetV2<'info>>,
        params: CreateFaucetV2Params
    ) -> Result<()> {
        actions::create_faucet_v2(ctx, params)
    }
    pub fn new_tree_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, AddNewTreeV2<'info>>
    ) -> Result<()> {
        actions::add_new_tree_v2(ctx)
    }
    pub fn edit_faucet_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateFaucetV2<'info>>,
        params: UpdateFaucetV2Params
    ) -> Result<()> {
        actions::update_faucet_v2(ctx, params)
    }
    // MINT
    pub fn mint_layer<'info>(
        ctx: Context<'_, '_, '_, 'info, MintLayerMap<'info>>,
        params: MintLayerMapParams
    ) -> Result<()> {
        actions::mint_layer_map(ctx, params)
    }
    pub fn mint_supply<'info>(
        ctx: Context<'_, '_, '_, 'info, MintSupplyMap<'info>>,
    ) -> Result<()> {
        actions::mint_supply_map(ctx)
    }
    pub fn mint_open<'info>(
        ctx: Context<'_, '_, '_, 'info, MintOpenMap<'info>>,
        params: MintOpenMapParams
    ) -> Result<()> {
        actions::mint_open_map(ctx, params)
    }
    // BACKGROUND 
    // pub fn set_bg_color<'info>( 
    //     ctx: Context<'_, '_, '_, 'info, SetBgColor<'info>>,
    //     params: SetBgColorParams
    // ) -> Result<()> {
    //     actions::set_bg_color(ctx, params)
    // }
    pub fn close_background<'info>(
        ctx: Context<'_, '_, '_, 'info, CloseBackground<'info>>,
        params: CloseBackgroundParams
    ) -> Result<()> {
        actions::close_bg(ctx, params)
    }
    // ADDITIONAL ASSETS
    pub fn create_additional_assets<'info>(
        ctx: Context<'_, '_, '_, 'info, InitAdditionalAssets<'info>>,
        params: InitAdditionalAssetsParams
    ) -> Result<()> {
        actions::init_additional_assets(ctx, params)
    }
    pub fn close_additional_assets<'info>(
        ctx: Context<'_, '_, '_, 'info, DeleteAdditionalAssets<'info>>,
        params: DeleteAdditionalAssetsParams
    ) -> Result<()> {
        actions::delete_additional_assets(ctx, params)
    }
    pub fn add_background<'info>(
        ctx: Context<'_, '_, '_, 'info, AddBg<'info>>,
        params: AddBgParams
    ) -> Result<()> {
        actions::add_bg(ctx, params)
    }
    pub fn remove_background<'info>(
        ctx: Context<'_, '_, '_, 'info, RemoveBg<'info>>,
        params: RemoveBgParams
    ) -> Result<()> {
        actions::remove_bg(ctx, params)
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