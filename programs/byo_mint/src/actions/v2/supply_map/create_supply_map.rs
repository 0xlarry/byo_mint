use crate::*;

// *********************************
// CREATE LAYER MAP
// *********************************
pub fn create_supply_map(ctx: Context<CreateSupplyMap>, params: CreateSupplyMapParams) -> Result<()> {
    *ctx.accounts.supply_map = SupplyMap::new(
        ctx.accounts.auth.key(),
        params.seller_fee_basis_points,
        params.symbol,
        params.uri_prefix,
        &params.creators,
        params.items
    ).unwrap();
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateSupplyMapParams {
    seller_fee_basis_points: u16,
    uri_prefix: String,
    symbol: String,
    creators: Vec<(Pubkey, u8)>,
    items: Vec<Item>
}

#[derive(Accounts)]
#[instruction(params: CreateSupplyMapParams)]
pub struct CreateSupplyMap<'info> {
    #[account(mut)]
    pub auth: Signer<'info>,
    #[account(
        init,
        space=SupplyMap::LEN,
        payer = auth,
        seeds=[b"supply", auth.key().as_ref(), params.symbol.as_bytes()],
        bump
    )]
    pub supply_map: Account<'info, SupplyMap>,
    pub system_program: Program<'info, System>
}
