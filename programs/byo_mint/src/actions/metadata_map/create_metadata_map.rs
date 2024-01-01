use crate::*;

// *********************************
// CREATE FAUCET
// *********************************
pub fn create_metadata_map(ctx: Context<CreateMetadataMap>, params: CreateMetadataMapParams) -> Result<()> {
    *ctx.accounts.metadata_map = MetadataMap::new(
        ctx.accounts.auth.key(),
        params.layers,
        params.seller_fee_basis_points,
        params.symbol,
        params.uri_prefix
    ).unwrap();
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateMetadataMapParams {
    seller_fee_basis_points: u16,
    layers: [u8; 10],
    uri_prefix: String,
    symbol: String
}

#[derive(Accounts)]
#[instruction(params: CreateMetadataMapParams)]
pub struct CreateMetadataMap<'info> {
    #[account(mut)]
    pub auth: Signer<'info>,
    #[account(
        init,
        space=MetadataMap::LEN,
        payer = auth,
        seeds=[auth.key().as_ref(), params.symbol.as_bytes()],
        bump
    )]
    pub metadata_map: Account<'info, MetadataMap>,
    pub system_program: Program<'info, System>
}
