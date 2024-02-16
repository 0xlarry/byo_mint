use crate::*;
use anchor_lang::{
    prelude::AccountInfo,
    solana_program::pubkey::Pubkey
};
use mpl_bubblegum::{utils::get_asset_id, instructions::TransferCpiBuilder};

#[derive(Accounts)]
#[instruction(params: AddBgTokenParams)]
pub struct AddBgToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, // cNFT owner
    #[account(
        mut,
        seeds=["bg".as_ref(), get_asset_id(&merkle_tree.key(), params.nonce).as_ref()],
        bump
    )]
    pub background: Account<'info, Background>, // escrow account (cNFT receiver)
    /// CHECK: This account is modified in the downstream program
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub bg_tree_config: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account is modified in the downstream program
    pub bg_merkle_tree: UncheckedAccount<'info>,
    /// CHECK: log wrapper program
    pub log_wrapper: UncheckedAccount<'info>,
    /// CHECK: compression program
    pub compression_program: UncheckedAccount<'info>,
    /// CHECK: buggle gum program
    pub bubblegum_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AddBgTokenParams {
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
    proof_len: u8,              
    bg_root: [u8; 32],
    bg_data_hash: [u8; 32],
    bg_creator_hash: [u8; 32],
    bg_nonce: u64,
    bg_index: u32,
    bg_proof_len: u8,           // BG proof passed in second!!
    bg_name: String,
    bg_creator: Pubkey,
    bg_uri: String,
}

pub fn add_bg_token<'info>(ctx: Context<'_, '_, '_, 'info, AddBgToken<'info>>, params: AddBgTokenParams) -> Result<()> {
    // require bg account string is not a pub key (already holds bg)
    require!(ctx.accounts.background.color_or_asset_id.len() < 44, ByomError::BackgroundTokenAlreadyExists);
    let bg_asset_id = get_asset_id(&ctx.accounts.bg_merkle_tree.key(), params.bg_nonce);
    ctx.accounts.background.color_or_asset_id = bg_asset_id.to_string();
    msg!("SET BG ASSET ID");

    // find proofs
    let proof = &ctx.remaining_accounts[0..(params.proof_len) as usize];
    let bg_proof = &ctx.remaining_accounts[(params.proof_len) as usize..(params.proof_len + params.bg_proof_len) as usize];

    // require signer owns the cNFT that is getting asset
    check_cnft_owner(
        &ctx.accounts.signer.to_account_info(), 
        &&ctx.accounts.merkle_tree.to_account_info(), 
        &&ctx.accounts.compression_program.to_account_info(),
        proof, 
        params.root,
        params.data_hash, 
        params.creator_hash,
        params.nonce, 
        params.index
    )?;

    // collection gate
    validate_metadata(params.bg_data_hash, params.bg_name, params.bg_uri, params.bg_creator)?;

    // transfer bg asset
    let remaining_accs: Vec<(&AccountInfo<'info>, bool, bool)> = bg_proof.iter()
        .map(|x| (
            x,
            false,
            false
        )).collect();
    TransferCpiBuilder::new(
        &ctx.accounts.bubblegum_program.to_account_info()
    )
        .tree_config(&ctx.accounts.bg_tree_config.to_account_info())
        .leaf_owner(&ctx.accounts.signer.to_account_info(), true)
        .leaf_delegate(&ctx.accounts.signer.to_account_info(), true)
        .new_leaf_owner(&ctx.accounts.background.to_account_info())
        .merkle_tree(&ctx.accounts.bg_merkle_tree.to_account_info())
        .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
        .compression_program(&ctx.accounts.compression_program.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .root(params.bg_root)
        .data_hash(params.bg_data_hash)
        .creator_hash(params.bg_creator_hash)
        .nonce(params.bg_nonce)
        .index(params.bg_index)
        .add_remaining_accounts(&remaining_accs)
        .invoke()?;

    Ok(())
}