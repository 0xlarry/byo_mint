
use crate::*;
use mpl_bubblegum::utils::get_asset_id;

// *********************************
// SET BG COLOR
// *********************************
pub fn set_bg_color<'info>(ctx: Context<'_, '_, '_, 'info, SetBgColor<'info>>, params: SetBgColorParams) -> Result<()> {
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
    )?;
    require!(is_valid_hex_color(&params.bg_color), ByomError::InvalidBackgroundColor);
    ctx.accounts.background.color_or_asset_id = params.bg_color;
    Ok(())
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetBgColorParams {
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
    layers: [u8; 10],
    bg_color: String,
}

#[derive(Accounts)]
#[instruction(params: SetBgColorParams)]
pub struct SetBgColor<'info> {
    #[account(mut)]
    pub leaf_owner: Signer<'info>,
    #[account(
        init_if_needed,
        payer = leaf_owner,
        space=Background::LEN,
        seeds=["bg".as_ref(), get_asset_id(&merkle_tree.key(), params.nonce).as_ref()],
        bump
    )]
    pub background: Account<'info, Background>,
    /// CHECK: This account is ok
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: compression program
    pub compression_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}