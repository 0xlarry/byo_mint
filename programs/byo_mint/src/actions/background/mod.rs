pub mod set_bg_color;
pub use set_bg_color::*;
pub mod close_bg;
pub use close_bg::*;
pub const TRANSFER_DISCRIMINATOR: &'static [u8;8] = &[163, 52, 200, 231, 140, 3, 69, 186];
pub mod add_bg_token;
pub use add_bg_token::*;
pub mod remove_bg_token;
pub use remove_bg_token::*;