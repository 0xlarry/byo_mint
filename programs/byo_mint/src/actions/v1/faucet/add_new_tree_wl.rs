
use crate::*;

use anchor_lang::solana_program::pubkey::Pubkey;
use mpl_bubblegum::instructions::CreateTreeConfigCpiBuilder;

// The program will support only trees of the following parameters:
const MAX_TREE_DEPTH: u32 = 14;
const MAX_TREE_BUFFER_SIZE: u32 = 64;
// this corresponds to account with a canopy depth 11.
// If you need the tree parameters to be dynamic, you can use the following function:
// fn tree_bytes_size() -> usize {
//     const CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1: usize = 2 + 54;
//     let merkle_tree_size = size_of::<ConcurrentMerkleTree<14, 64>>();
//     msg!("merkle tree size: {}", merkle_tree_size);
//     let canopy_size = ((2 << 9) - 2) * 32;
//     msg!("canopy size: {}", canopy_size);
//     CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1 + merkle_tree_size + (canopy_size as usize)
// }

// *********************************
// ADD NEW TREE TO FAUCET
// *********************************
const REQUIRED_TREE_ACCOUNT_SIZE: usize = 162_808;
pub fn add_new_tree_wl(ctx: Context<AddNewTreeWl>) -> Result<()> {
    msg!("-- Initializing merkle tree");
    require!(ctx.accounts.faucet.authority == ctx.accounts.faucet_auth.key(), ByomError::InvalidAuthority);
    require!(ctx.accounts.merkle_tree.data.borrow().len() == REQUIRED_TREE_ACCOUNT_SIZE, ByomError::InvalidTree);
    let signer_seeds: &[&[&[u8]]] = &[&[
        "wl".as_bytes(),
        ctx.accounts.faucet.authority.as_ref(),
        ctx.accounts.faucet.metadata_map.as_ref(),
        &[ctx.accounts.faucet.bump],
    ]];

    // hard coded for space for 10k collection => 1.134 SOL
    CreateTreeConfigCpiBuilder::new(
        &ctx.accounts.bubblegum_program.to_account_info(),
    )
        .tree_config(&ctx.accounts.tree_config.to_account_info())
        .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
        .payer(&&ctx.accounts.faucet_auth.to_account_info())
        .tree_creator(&&ctx.accounts.faucet.to_account_info())
        .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
        .compression_program(&ctx.accounts.compression_program.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .max_depth(MAX_TREE_DEPTH)
        .max_buffer_size(MAX_TREE_BUFFER_SIZE)
        .invoke_signed(signer_seeds)?;

    ctx.accounts.faucet.merkle_tree = ctx.accounts.merkle_tree.key();
    msg!("-- Updated Merkle Tree on Faucet");
    Ok(())
}

#[derive(Accounts)]
pub struct AddNewTreeWl<'info> {
    #[account(mut)]
    pub faucet_auth: Signer<'info>,
    #[account(mut)]
    pub faucet: Account<'info, FaucetWl>,
    /// CHECK: This account must be all zeros
    #[account(
        zero,
        signer
    )]
    pub merkle_tree: AccountInfo<'info>,
    /// CHECK: This account is checked in the instruction
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,
    // programs
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
}
