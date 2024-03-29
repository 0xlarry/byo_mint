use anchor_lang::prelude::*;

#[error_code]
pub enum ByomError {
    #[msg("Too many items")]
    TooManyLayers,
    #[msg("Supply cap")]
    SupplyCap,
    #[msg("Item not found")]
    ItemNotFound,
    #[msg("Invalid Rarity")]
    InvalidRoyalty,
    #[msg("Invalid Authority")]
    InvalidAuthority,
    #[msg("Nothing to mint")]
    NothingToMint,
    #[msg("Invalid Metadata")]
    InvalidMetadata,
    #[msg("Invalid Account")]
    InvalidAccount,
    #[msg("Merkle Tree is not full")]
    TreeNotFull,
    #[msg("Invalid Supply")]
    InvalidSupply,
    #[msg("Invalid Tree")]
    InvalidTree,
    #[msg("Invalid Variant")]
    InvalidVariant,
    #[msg("Invalid Symbol")]
    InvalidSymbol,
    #[msg("Invalid URI")]
    InvalidUri,
    #[msg("Invalid Background Color")]
    InvalidBackgroundColor,
    #[msg("Invalid Collection")]
    InvalidCollection,
    #[msg("Invalid Owner")]
    InvalidOwner,
}