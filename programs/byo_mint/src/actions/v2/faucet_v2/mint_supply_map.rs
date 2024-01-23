use crate::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::system_instruction;
use mpl_bubblegum::instructions::MintToCollectionV1CpiBuilder;
use mpl_bubblegum::types::{Collection, MetadataArgs, TokenProgramVersion, TokenStandard};


// *********************************
// MINT cNFT FROM FAUCET
// *********************************
pub fn mint_supply_map(ctx: Context<MintSupplyMap>) -> Result<()> {
    // checks
    require!(ctx.accounts.faucet.merkle_tree == ctx.accounts.merkle_tree.key(), ByomError::InvalidAccount);
    require!(ctx.accounts.supply_map.key() == ctx.accounts.faucet.supply_map, ByomError::InvalidAccount);
    require!(ctx.accounts.faucet.current_supply < ctx.accounts.faucet.supply_cap, ByomError::SupplyCap);

    // pay fees
    invoke(
        &system_instruction::transfer(&ctx.accounts.minter.key(), &ctx.accounts.faucet.key(), ctx.accounts.faucet.mint_price), 
    &[
        ctx.accounts.minter.to_account_info(),
        ctx.accounts.faucet.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ])?;
    msg!("PAID FEE");

    // choose item
    let item_to_mint = SupplyMap::select_item(&mut ctx.accounts.supply_map, &ctx.accounts.clock).unwrap();
    
    // mint cnft
    let supply_map = &mut ctx.accounts.supply_map;
    let signer_seeds: &[&[&[u8]]] = &[&[
        ctx.accounts.faucet.authority.as_ref(),
        ctx.accounts.faucet.supply_map.as_ref(),
        &[ctx.accounts.faucet.bump],
    ]];
    MintToCollectionV1CpiBuilder::new(
        &ctx.accounts.bubblegum_program.to_account_info(),
    )
        .tree_config(&ctx.accounts.tree_config.to_account_info())
        .leaf_owner(&ctx.accounts.leaf_owner.to_account_info())
        .leaf_delegate(&ctx.accounts.leaf_owner.to_account_info())
        .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
        .payer(&ctx.accounts.minter.to_account_info())
        .tree_creator_or_delegate(&ctx.accounts.faucet.to_account_info())
        .collection_authority(&ctx.accounts.faucet.to_account_info())
        .collection_authority_record_pda(Some(&ctx.accounts.bubblegum_program.to_account_info()))
        .collection_mint(&ctx.accounts.collection_mint.to_account_info())
        .collection_metadata(&ctx.accounts.collection_metadata.to_account_info())
        .collection_edition(&ctx.accounts.edition_account.to_account_info())
        .bubblegum_signer(&ctx.accounts.bubblegum_signer.to_account_info())
        .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
        .compression_program(&ctx.accounts.compression_program.to_account_info())
        .token_metadata_program(&ctx.accounts.token_metadata_program.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .metadata( MetadataArgs {
                name: format!("{}", item_to_mint.name),
                symbol: supply_map.symbol.clone(),
                uri: format!("{}/{}.json", supply_map.uri_prefix, item_to_mint.json_uri_suffix),
                creators: build_creators(supply_map.creators.clone(), ctx.accounts.minter.key()),
                seller_fee_basis_points: supply_map.seller_fee_basis_points,
                primary_sale_happened: false,
                is_mutable: false,
                edition_nonce: Some(0),
                uses: None,
                collection: Some(Collection {verified: false, key: ctx.accounts.collection_mint.key()}),
                token_program_version: TokenProgramVersion::Original,
                token_standard: Some(TokenStandard::NonFungible),
            }
        ).invoke_signed(signer_seeds).unwrap();

    // increase supply
    ctx.accounts.faucet.current_supply += 1;
    msg!("** MINTED");

    Ok(())
}

#[derive(Accounts)]
pub struct MintSupplyMap<'info> {
    #[account(mut)]
    pub minter: Signer<'info>,
    #[account(mut)]
    pub faucet: Box<Account<'info, FaucetV2>>,
    pub supply_map: Box<Account<'info, SupplyMap>>,
    /// CHECK: This account is checked in the instruction
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub leaf_owner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: unsafe
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the instruction
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the instruction
    pub edition_account: UncheckedAccount<'info>,
    /// CHECK: This is just used as a signing PDA.
    pub bubblegum_signer: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub token_metadata_program: Program<'info, MplTokenMetadata>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>
}
