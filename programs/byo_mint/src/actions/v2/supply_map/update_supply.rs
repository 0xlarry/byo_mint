use crate::*;

// *********************************
// UPDATE LAYER MAP
// *********************************
pub fn update_supply(ctx: Context<UpdateSupply>, params: UpdateSupplyParams) -> Result<()> {
    require!(ctx.accounts.auth.key() == ctx.accounts.supply_map.authority, ByomError::InvalidAuthority);
    Item::verify_items(params.items.clone())?;
    ctx.accounts.supply_map.items = params.items;
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateSupplyParams {
    items: Vec<Item>
}

#[derive(Accounts)]
#[instruction(params: UpdateSupplyParams)]
pub struct UpdateSupply<'info> {
    #[account(mut)]
    pub auth: Signer<'info>,
    #[account(mut)]
    pub supply_map: Account<'info, SupplyMap>,
}
