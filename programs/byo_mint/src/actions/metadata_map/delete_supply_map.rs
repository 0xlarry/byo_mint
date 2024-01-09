use crate::*;

// *******************
// DELETE SUPPLY MAP
// *******************
#[derive(Accounts)]
pub struct DeleteSupplyMap<'info> {
    #[account(
        mut,
        close = auth
    )]
    pub md_map: Account<'info, SupplyMap>,
    #[account(mut)]
    pub auth: Signer<'info>,
    /// CHECK: system program is ok
    pub system_program: AccountInfo<'info>
}

pub fn delete_supply_map(ctx: Context<DeleteSupplyMap>) -> Result<()> {
    require!(ctx.accounts.auth.key() == ctx.accounts.md_map.authority, ByomError::InvalidAuthority);
    Ok(())
}