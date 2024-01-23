use crate::*;
pub mod v1;
pub use v1::*;
pub mod v2;
pub use v2::*;
pub mod util;
pub use util::*;

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