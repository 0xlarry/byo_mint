use crate::*;

// *********************************
// CREATE FAUCET
// *********************************
pub fn init_metadata_map_v2(ctx: Context<InitMetadataMapV2>, params: InitMetadataMapV2Params) -> Result<()> {
    *ctx.accounts.metadata_map = MetadataMapV2::new(
        ctx.accounts.auth.key(),
        params.layers,
        params.seller_fee_basis_points,
        params.symbol,
        params.uri_prefix,
        params.creators
    ).unwrap();
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitMetadataMapV2Params {
    seller_fee_basis_points: u16,
    layers: [u8; 10],
    uri_prefix: String,
    symbol: String,
    creators: Vec<ByoCreator>,
}

#[derive(Accounts)]
#[instruction(params: InitMetadataMapV2Params)]
pub struct InitMetadataMapV2<'info> {
    #[account(mut)]
    pub auth: Signer<'info>,
    #[account(
        init,
        space=MetadataMapV2::LEN,
        payer = auth,
        seeds=[auth.key().as_ref(), params.symbol.as_bytes()],
        bump
    )]
    pub metadata_map: Account<'info, MetadataMapV2>,
    pub system_program: Program<'info, System>
}
