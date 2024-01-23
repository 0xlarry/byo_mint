use crate::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, 
        create_metadata_accounts_v3, 
        CreateMasterEditionV3, 
        CreateMetadataAccountsV3, 
        Metadata,
        mpl_token_metadata::{types::{CollectionDetails, DataV2, Creator}, instructions::VerifyCreatorV1CpiBuilder},
    },
    token::{Mint, mint_to, MintTo, Token, TokenAccount},
};


// ***************************************************
// CREATE FAUCET
// ***************************************************
// TODO: require(merkle account size >= sizeof(max_supply))
pub fn create_faucet(ctx: Context<CreateFaucet>, params: CreateFaucetParams) -> Result<()> {
    // ************************************
    // set up faucet account
    require!(ctx.accounts.faucet_auth.key() == ctx.accounts.metadata_map.authority, ByomError::InvalidAuthority);
    ctx.accounts.faucet.authority = ctx.accounts.faucet_auth.key();
    ctx.accounts.faucet.collection_mint = ctx.accounts.mint.key();
    ctx.accounts.faucet.merkle_tree = Pubkey::default();
    ctx.accounts.faucet.metadata_map = ctx.accounts.metadata_map.key();
    ctx.accounts.faucet.current_supply = 0;
    ctx.accounts.faucet.supply_cap = params.supply_cap;
    ctx.accounts.faucet.mint_price = params.mint_price;
    ctx.accounts.faucet.bump = ctx.bumps.faucet;
    msg!("-- Set account data");
    
    let signer_seeds: &[&[&[u8]]] = &[&[
        ctx.accounts.faucet.authority.as_ref(),
        ctx.accounts.faucet.metadata_map.as_ref(),
        &[ctx.accounts.faucet.bump],
    ]];
    
    // ************************************
    // create collection mint account
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.associated_token_account.to_account_info(),
            authority: ctx.accounts.faucet.to_account_info(),
        },
        signer_seeds,
    );
    mint_to(cpi_context, 1)?;
    msg!("-- Created collection nft mint, {}", ctx.accounts.mint.key().to_string());

    // ************************************
    // create metadata account
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            metadata: ctx.accounts.metadata_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            mint_authority: ctx.accounts.faucet.to_account_info(),          // mint authority of collection metadata is faucet (pda)
            update_authority: ctx.accounts.faucet.to_account_info(),   // update authority to collection metadata is faucet authority (signer)
            payer: ctx.accounts.faucet_auth.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer_seeds
    );
    let data_v2 = DataV2 {
        name: params.collection_name,
        symbol: params.collection_symbol,
        uri: params.collection_uri,
        seller_fee_basis_points: 0,
        creators: Some(vec![
            Creator {address: ctx.accounts.faucet_auth.key().clone(), verified: false, share: 100},
            Creator {address: ctx.accounts.faucet.key().clone(), verified: true, share: 0},
        ]),
        collection: None,
        uses: None,
    };
    create_metadata_accounts_v3(
        cpi_context,
        data_v2,
        true,
        true,
        Some(CollectionDetails::V1 { size: 1 }),
    )?;
    VerifyCreatorV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .authority(&ctx.accounts.faucet_auth.to_account_info())
        .metadata(&ctx.accounts.metadata_account.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .sysvar_instructions(&ctx.accounts.sysvar_instructions.to_account_info())
        .invoke()?;
    msg!("-- Created collection nft metadata {}", ctx.accounts.metadata_account.key().to_string());

    // ************************************
    // create master edition account
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMasterEditionV3 {
            edition: ctx.accounts.master_edition_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            mint_authority: ctx.accounts.faucet.to_account_info(),          // mint authority of collection metadata is faucet (pda)
            update_authority: ctx.accounts.faucet.to_account_info(),   // update authority to collection metadata is faucet authority (signer)
            payer: ctx.accounts.faucet_auth.to_account_info(),
            metadata: ctx.accounts.metadata_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer_seeds
    );
    create_master_edition_v3(cpi_context, Some(0))?;
    msg!("-- Created collection nft edition");
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateFaucetParams {
    collection_name: String,
    collection_symbol: String,
    collection_uri: String,
    mint_price: u64,
    supply_cap: u64
}

#[derive(Accounts)]
pub struct CreateFaucet<'info> {
    #[account(mut)]
    pub faucet_auth: Signer<'info>,
    #[account(
        init,
        space=Faucet::LEN,
        payer = faucet_auth,
        seeds=[faucet_auth.key().as_ref(), metadata_map.key().as_ref()],
        bump
    )]
    pub faucet: Account<'info, Faucet>,
    pub metadata_map: Account<'info, MetadataMap>,
    #[account(
        init,
        payer = faucet_auth,
        mint::decimals = 0,
        mint::authority = faucet.key(),
        mint::freeze_authority = faucet.key(),
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = faucet_auth,
        associated_token::mint = mint,
        associated_token::authority = faucet
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    /// CHECK - address
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,
    /// CHECK: address
    #[account(mut)]
    pub master_edition_account: AccountInfo<'info>,
    // programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    /// CHECK: system program is ok
    pub sysvar_instructions: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}
