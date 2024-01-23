use crate::*;

// *********************************
// CREATE LAYER MAP
// *********************************
pub fn create_layer_map(ctx: Context<CreateLayerMap>, params: CreateLayerMapParams) -> Result<()> {
    *ctx.accounts.layer_map = LayerMap::new(
        ctx.accounts.auth.key(),
        params.seller_fee_basis_points,
        params.symbol,
        params.uri_prefix,
        &params.creators,
        params.layers
    ).unwrap();
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateLayerMapParams {
    seller_fee_basis_points: u16,
    uri_prefix: String,
    symbol: String,
    creators: Vec<(Pubkey, u8)>,
    layers: [u8; 10]
}

#[derive(Accounts)]
#[instruction(params: CreateLayerMapParams)]
pub struct CreateLayerMap<'info> {
    #[account(mut)]
    pub auth: Signer<'info>,
    #[account(
        init,
        space=LayerMap::LEN,
        payer = auth,
        seeds=[b"layer", auth.key().as_ref(), params.symbol.as_bytes()],
        bump
    )]
    pub layer_map: Account<'info, LayerMap>,
    pub system_program: Program<'info, System>
}
