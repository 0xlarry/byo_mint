use crate::*;

// *******************
// DELETE METADATA MAP
// *******************
#[derive(Accounts)]
pub struct CloseMetadataMapV2<'info> {
    #[account(
        mut,
        close = auth
    )]
    pub metadata_map: Account<'info, MetadataMapV2>,
    #[account(mut)]
    pub auth: Signer<'info>,
    /// CHECK: system program is ok
    pub system_program: AccountInfo<'info>
}

pub fn close_metadata_map_v2(ctx: Context<CloseMetadataMapV2>) -> Result<()> {
    require!(ctx.accounts.auth.key() == ctx.accounts.metadata_map.authority, ByomError::InvalidAuthority);
    Ok(())
}