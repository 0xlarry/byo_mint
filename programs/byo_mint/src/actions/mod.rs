use crate::*;
pub mod v1;
pub use v1::*;
pub mod v2;
pub use v2::*;
pub mod util;
pub use util::*;

pub const TRANSFER_DISCRIMINATOR: &'static [u8;8] = &[163, 52, 200, 231, 140, 3, 69, 186];

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