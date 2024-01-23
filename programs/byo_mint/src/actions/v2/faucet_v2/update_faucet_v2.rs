
use crate::*;

// *********************************
// UPDATE FAUCET
// *********************************
pub fn update_faucet_v2(ctx: Context<UpdateFaucetV2>, params: UpdateFaucetV2Params) -> Result<()> {
    require!(ctx.accounts.faucet_auth.key() == ctx.accounts.faucet.authority, ByomError::InvalidAccount);
    match params.supply_cap {
        Some(new_supply_cap) => {
            require!(ctx.accounts.faucet.supply_cap < new_supply_cap, ByomError::InvalidSupply);
            ctx.accounts.faucet.supply_cap = new_supply_cap;
        }
        None => {}
    }

    match params.mint_price {
        Some(new_mint_price) => {
            ctx.accounts.faucet.mint_price = new_mint_price;
        }
        None => {}
    }

    match &ctx.accounts.layer_map {
        Some(x) => {
            require!(x.authority == ctx.accounts.faucet.authority, ByomError::InvalidAuthority);
            ctx.accounts.faucet.layer_map = x.key();
        },
        None => {}
    };

    match &ctx.accounts.supply_map {
        Some(x) => {
            require!(x.authority == ctx.accounts.faucet.authority, ByomError::InvalidAuthority);
            ctx.accounts.faucet.supply_map = x.key();
        },
        None => {}
    };
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateFaucetV2Params {
    supply_cap: Option<u64>,
    mint_price: Option<u64>,
}

#[derive(Accounts)]
pub struct UpdateFaucetV2<'info> {
    #[account(mut)]
    pub faucet_auth: Signer<'info>,
    pub layer_map: Option<Account<'info, LayerMap>>,
    pub supply_map: Option<Account<'info, SupplyMap>>,
    #[account(mut)]
    pub faucet: Account<'info, FaucetV2>,
    pub system_program: Program<'info, System>,
}