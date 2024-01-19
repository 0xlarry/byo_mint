
use crate::*;

pub fn close_background<'info>(ctx: Context<'_, '_, '_, 'info, CloseBackground<'info>>, params: CloseBackgroundParams) -> Result<()> {
    check_cnft_owner(
        &ctx.accounts.leaf_owner.to_account_info(), 
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
pub struct CloseBackgroundParams {
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
}

#[derive(Accounts)]
pub struct CloseBackground<'info> {
    #[account(mut)]
    pub leaf_owner: Signer<'info>,
    #[account(mut, close = leaf_owner)]
    pub background: Account<'info, Background>,
    /// CHECK: This account is ok
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: compression program
    pub compression_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}