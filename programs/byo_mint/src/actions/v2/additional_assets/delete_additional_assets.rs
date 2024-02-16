
use crate::*;

pub fn delete_additional_assets<'info>(ctx: Context<'_, '_, '_, 'info, DeleteAdditionalAssets<'info>>, params: DeleteAdditionalAssetsParams) -> Result<()> {
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
    )
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DeleteAdditionalAssetsParams {
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
}

#[derive(Accounts)]
pub struct DeleteAdditionalAssets<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, close = signer)]
    pub additional_assets: Account<'info, AdditionalAssets>,
    /// CHECK: This account is ok
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: compression program
    pub compression_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}