use crate::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token::TokenAccount;
use mpl_bubblegum::instructions::MintToCollectionV1CpiBuilder;
use mpl_bubblegum::types::{Creator, Collection, MetadataArgs, TokenProgramVersion, TokenStandard};


// *********************************
// MINT cNFT FROM FAUCET
// *********************************
pub fn mint_cnft_wl(ctx: Context<MintCnftWl>, params: MintCnftWlParams) -> Result<()> {
    // checks
    require!(ctx.accounts.faucet.merkle_tree == ctx.accounts.merkle_tree.key(), ByomError::InvalidAccount);
    require!(ctx.accounts.metadata_map.key() == ctx.accounts.faucet.metadata_map, ByomError::InvalidAccount);
    require!(ctx.accounts.faucet.current_supply < ctx.accounts.faucet.supply_cap, ByomError::SupplyCap);

    // token gate
    FaucetWl::assert_wl(&mut ctx.accounts.faucet, ctx.accounts.minter.key(), ctx.accounts.token_account.clone(), ctx.accounts.metadata.clone())?;

    // bg color does not factor into the byo_mint pda... thus same color can be used on different layer combos
    match params.bg_color {
        Some(bgc) => {
            require!(is_valid_hex_color(bgc.as_str()), ByomError::InvalidBackgroundColor);
        },
        None => {}
    }

    // validate layers
    MetadataMap::validate_input_layers(&mut ctx.accounts.metadata_map, params.layers.clone())?;

    // pay fees
    invoke(
        &system_instruction::transfer(&ctx.accounts.minter.key(), &ctx.accounts.faucet.key(), ctx.accounts.faucet.mint_price), 
    &[
        ctx.accounts.minter.to_account_info(),
        ctx.accounts.faucet.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ])?;
    msg!("PAID FEE");
    
    // mint cnft
    let metadata_map = &mut ctx.accounts.metadata_map;
    let signer_seeds: &[&[&[u8]]] = &[&[
        "wl".as_bytes(),
        ctx.accounts.faucet.authority.as_ref(),
        ctx.accounts.faucet.metadata_map.as_ref(),
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
                name: format!("{} {}", metadata_map.symbol, ctx.accounts.faucet.current_supply),
                symbol: metadata_map.symbol.clone(),
                uri: format!("{}/{}.json", metadata_map.uri_prefix, ctx.accounts.byo_mint.key().to_string()),
                creators: vec![
                    Creator {address: ctx.accounts.faucet.key().clone(), verified: true, share: 50},
                    Creator {address: ctx.accounts.minter.key().clone(), verified: true, share: 50},
                ],
                seller_fee_basis_points: metadata_map.seller_fee_basis_points,
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MintCnftWlParams {
    layers: [u8; 10],
    bg_color: Option<String>
}

#[derive(Accounts)]
#[instruction(params: MintCnftWlParams)]
pub struct MintCnftWl<'info> {
    #[account(mut)]
    pub minter: Signer<'info>,
    #[account(mut)]
    pub faucet: Box<Account<'info, FaucetWl>>,
    pub metadata_map: Box<Account<'info, MetadataMap>>,
    #[account(
        init,
        space=ByoMint::LEN,
        payer = minter,
        seeds=[metadata_map.key().as_ref(), &params.layers],
        bump
    )]
    pub byo_mint: Account<'info, ByoMint>,
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
    // COLLECTION TOKEN GATING
    #[account(
        init,
        space=WlMint::LEN,
        payer = minter,
        seeds=[metadata_map.key().as_ref(), token_account.mint.as_ref()],
        bump
    )]
    pub wl_mint: Account<'info, WlMint>,
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: metadata is ok
    pub metadata: AccountInfo<'info>
}
