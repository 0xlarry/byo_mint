use crate::*;
pub mod faucet;
pub use faucet::*;
pub mod metadata_map;
pub use metadata_map::*;
pub mod util;
pub use util::*;
pub mod v2;
pub use v2::*;

pub struct TokenProgram;
impl anchor_lang::Id for TokenProgram {
    fn id() -> Pubkey {
        anchor_spl::token::ID
    }
}
pub struct MplMetadataProgram;
impl anchor_lang::Id for MplMetadataProgram {
    fn id() -> Pubkey {
        mpl_token_metadata::ID
    }
}