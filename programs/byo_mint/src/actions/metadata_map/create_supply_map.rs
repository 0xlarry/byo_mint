use crate::*;

// *********************************
// CREATE SUPPLY MAP
// *********************************
pub fn create_supply_map(ctx: Context<CreateSupplyMap>, params: CreateSupplyMapParams) -> Result<()> {
    *ctx.accounts.metadata_map = SupplyMap::new(
        ctx.accounts.auth.key(),
        params.variants.clone(),
        params.seller_fee_basis_points,
        params.uri_prefix,
        params.name
    ).unwrap();
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateSupplyMapParams {
    seller_fee_basis_points: u16,
    variants: Vec<Variant>,
    uri_prefix: String,
    name: String
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
        seeds=[auth.key().as_ref(), &params.name.as_bytes()],
        bump
    )]
    pub metadata_map: Account<'info, SupplyMap>,
    pub system_program: Program<'info, System>
}
