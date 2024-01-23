use crate::*;

// *********************************
// CREATE LAYER MAP
// *********************************
pub fn create_open_map(ctx: Context<CreateOpenMap>, params: CreateOpenMapParams) -> Result<()> {
    *ctx.accounts.open_map = OpenMap::new(
        ctx.accounts.auth.key(),
        params.seller_fee_basis_points,
        params.symbol,
        &params.creators,
    ).unwrap();
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateOpenMapParams {
    seller_fee_basis_points: u16,
    symbol: String,
    creators: Vec<ByoCreator>,
}

#[derive(Accounts)]
#[instruction(params: CreateOpenMapParams)]
pub struct CreateOpenMap<'info> {
    #[account(mut)]
    pub auth: Signer<'info>,
    #[account(
        init,
        space=OpenMap::LEN,
        payer = auth,
        seeds=[b"open", auth.key().as_ref(), params.symbol.as_bytes()],
        bump
    )]
    pub open_map: Account<'info, OpenMap>,
    pub system_program: Program<'info, System>
}
