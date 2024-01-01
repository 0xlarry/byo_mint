use crate::*;

// *******************
// DELETE METADATA MAP
// *******************
#[derive(Accounts)]
pub struct DeleteMetadataMap<'info> {
    #[account(
        mut,
        close = auth
    )]
    pub metadata_map: Account<'info, MetadataMap>,
    #[account(mut)]
    pub auth: Signer<'info>,
    /// CHECK: system program is ok
    pub system_program: AccountInfo<'info>
}

pub fn delete_metadata_map(ctx: Context<DeleteMetadataMap>) -> Result<()> {
    require!(ctx.accounts.auth.key() == ctx.accounts.metadata_map.authority, ByomError::InvalidAuthority);
    Ok(())
}