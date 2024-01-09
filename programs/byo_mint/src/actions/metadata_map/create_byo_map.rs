use crate::*;

// *********************************
// CREATE FAUCET
// *********************************
pub fn create_byo_map(ctx: Context<CreateByoMap>, params: CreateByoMapParams) -> Result<()> {
    *ctx.accounts.metadata_map = ByoMap::new(
        ctx.accounts.auth.key(),
        params.layers,
        params.seller_fee_basis_points,
        params.symbol,
        params.uri_prefix
    ).unwrap();
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateByoMapParams {
    seller_fee_basis_points: u16,
    layers: [u8; 10],
    uri_prefix: String,
    symbol: String
}

#[derive(Accounts)]
#[instruction(params: CreateByoMapParams)]
pub struct CreateByoMap<'info> {
    #[account(mut)]
    pub auth: Signer<'info>,
    #[account(
        init,
        space=ByoMap::LEN,
        payer = auth,
        seeds=[auth.key().as_ref(), params.symbol.as_bytes()],
        bump
    )]
    pub metadata_map: Account<'info, ByoMap>,
    pub system_program: Program<'info, System>
}
