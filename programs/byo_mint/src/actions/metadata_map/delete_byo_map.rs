use crate::*;

// *******************
// DELETE METADATA MAP
// *******************
#[derive(Accounts)]
pub struct DeleteByoMap<'info> {
    #[account(
        mut,
        close = auth
    )]
    pub metadata_map: Account<'info, ByoMap>,
    #[account(mut)]
    pub auth: Signer<'info>,
    /// CHECK: system program is ok
    pub system_program: AccountInfo<'info>
}

pub fn delete_byo_map(ctx: Context<DeleteByoMap>) -> Result<()> {
    require!(ctx.accounts.auth.key() == ctx.accounts.metadata_map.authority, ByomError::InvalidAuthority);
    Ok(())
}