
use crate::*;

// *********************************
// WITHDRAW FEES
// *********************************
pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
    require!(ctx.accounts.faucet_auth.key() == ctx.accounts.faucet.authority, ByomError::InvalidAccount);
    let total_lamports = ctx.accounts.faucet.to_account_info().lamports();
    let rent_exempt = Rent::default().minimum_balance(ctx.accounts.faucet.to_account_info().data_len());
    let fees = total_lamports - rent_exempt;
    **ctx.accounts.faucet_auth.to_account_info().try_borrow_mut_lamports()? += fees;
    **ctx.accounts.faucet.to_account_info().try_borrow_mut_lamports()? -= fees;
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(mut)]
    pub faucet_auth: Signer<'info>,
    #[account(mut)]
    pub faucet: Account<'info, Faucet>,
    pub system_program: Program<'info, System>,
}