
use crate::*;

// *********************************
// UPDATE FAUCET
// *********************************
pub fn update_faucet(ctx: Context<UpdateFaucet>, params: UpdateFaucetParams) -> Result<()> {
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
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateFaucetParams {
    supply_cap: Option<u64>,
    mint_price: Option<u64>
}

#[derive(Accounts)]
pub struct UpdateFaucet<'info> {
    #[account(mut)]
    pub faucet_auth: Signer<'info>,
    #[account(mut)]
    pub faucet: Account<'info, Faucet>,
    pub system_program: Program<'info, System>,
}