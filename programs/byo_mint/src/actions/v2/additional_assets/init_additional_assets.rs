
use crate::*;
use mpl_bubblegum::utils::get_asset_id;

pub fn init_additional_assets<'info>(ctx: Context<'_, '_, '_, 'info, InitAdditionalAssets<'info>>, params: InitAdditionalAssetsParams) -> Result<()> {
    check_cnft_owner(
        &ctx.accounts.signer.to_account_info(), 
        &&ctx.accounts.merkle_tree.to_account_info(), 
        &&ctx.accounts.compression_program.to_account_info(),
        ctx.remaining_accounts, 
        params.root,
        params.data_hash, 
        params.creator_hash,
        params.nonce, 
        params.index
    )?;
    *ctx.accounts.additional_assets = AdditionalAssets::new();
    Ok(())
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitAdditionalAssetsParams {
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
}

#[derive(Accounts)]
#[instruction(params: InitAdditionalAssetsParams)]
pub struct InitAdditionalAssets<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space=AdditionalAssets::LEN,
        seeds=["additional_assets".as_ref(), get_asset_id(&merkle_tree.key(), params.nonce).as_ref()],
        bump
    )]
    pub additional_assets: Account<'info, AdditionalAssets>,
    /// CHECK: This account is ok
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: compression program
    pub compression_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}