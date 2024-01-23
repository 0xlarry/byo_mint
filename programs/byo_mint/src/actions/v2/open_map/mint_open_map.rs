use crate::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token::{ TokenAccount, transfer, Transfer};
use mpl_bubblegum::instructions::MintToCollectionV1CpiBuilder;
use mpl_bubblegum::types::{Collection, MetadataArgs, TokenProgramVersion, TokenStandard};


// *********************************
// MINT cNFT FROM FAUCET
// *********************************
pub fn mint_open_map(ctx: Context<MintOpenMap>, params: MintOpenMapParams) -> Result<()> {
    // checks
    FaucetV2::mint_requirements(
        &mut ctx.accounts.faucet,
        ctx.accounts.merkle_tree.key(), 
        ctx.accounts.creator.key(), 
        None, 
        None, 
        Some(&mut *ctx.accounts.open_map),
    )?;

    // pay fees to FIRST CREATOR
    if ctx.accounts.faucet.mint_token == Pubkey::default() {
        invoke(
            &system_instruction::transfer(&ctx.accounts.minter.key(), &ctx.accounts.faucet.key(), ctx.accounts.faucet.mint_price), 
        &[
            ctx.accounts.minter.to_account_info(),
            ctx.accounts.creator.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ])?;
    } else {
        let minter_ta;
        match &ctx.accounts.minter_ta {
            Some(ta) => {minter_ta = ta;},
            None => {return Err(ByomError::InvalidAccount.into());}
        }
        let creator_ta;
        match &ctx.accounts.creator_ta {
            Some(ta) => {creator_ta = ta;},
            None => {return Err(ByomError::InvalidAccount.into());}
        }
        require!(creator_ta.owner == ctx.accounts.open_map.creators[0].address, ByomError::InvalidAccount);
        require!(
            minter_ta.mint == ctx.accounts.faucet.mint_token && creator_ta.mint == ctx.accounts.faucet.mint_token,
            ByomError::InvalidAccount
        );
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: minter_ta.to_account_info(),
                    to: creator_ta.to_account_info(),
                    authority: ctx.accounts.minter.to_account_info(),
                },
            ),
            ctx.accounts.faucet.mint_price,
        )?;
    }
    

    // choose item
    OpenMap::verify_metadata(params.json_uri.clone(), params.name.clone())?;
    
    // mint cnft
    let open_map = &mut ctx.accounts.open_map;
    let signer_seeds: &[&[&[u8]]] = &[&[
        ctx.accounts.faucet.authority.as_ref(),
        ctx.accounts.faucet.collection_mint.as_ref(),
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
                name: params.name,
                symbol: open_map.symbol.clone(),
                uri: params.json_uri,
                creators: build_creators(open_map.creators.clone(), ctx.accounts.minter.key()),
                seller_fee_basis_points: open_map.seller_fee_basis_points,
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
pub struct MintOpenMapParams {
    json_uri: String,
    name: String
}

#[derive(Accounts)]
pub struct MintOpenMap<'info> {
    #[account(mut)]
    pub minter: Signer<'info>,
    #[account(mut)]
    pub faucet: Box<Account<'info, FaucetV2>>,
    pub open_map: Box<Account<'info, OpenMap>>,
    /// CHECK: This account is checked in the instruction
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: unsafe
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub leaf_owner: AccountInfo<'info>,
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
    pub token_program: Program<'info, TokenProgram>,
    /// CHECK: creator public key to send platform fees
    #[account(mut)]
    pub creator: AccountInfo<'info>,
    #[account(mut)]
    pub creator_ta: Option<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub minter_ta: Option<Account<'info, TokenAccount>>,
}
